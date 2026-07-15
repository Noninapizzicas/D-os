//! CLI de Crawl4RS.
//!
//! ```text
//! crawl4rs crawl <url> [OPTIONS]   Procesa una URL (o un HTML local con --html-file)
//! crawl4rs deep  <url> [OPTIONS]   Crawl profundo siguiendo enlaces (BFS/DFS)
//! crawl4rs serve [OPTIONS]         Lanza el servidor API — Fase 6
//! crawl4rs config                  Muestra la configuración por defecto
//! ```

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use tracing_subscriber::EnvFilter;

use crawl4rs_core::{
    BrowserFetcher, CrawlConfig, Crawler, CssSelectorStrategy, DeepStrategy, ExtractionStrategy,
    FetchMode, FieldSpec, HttpFetcher, ResultCache, SemanticDensityStrategy, StaticFetcher,
    StealthConfig, StealthEngine,
};

#[derive(Parser)]
#[command(
    name = "crawl4rs",
    version,
    about = "Convierte cualquier página web en Markdown para LLMs."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Procesa una única URL.
    Crawl(CrawlArgs),
    /// Crawl profundo siguiendo enlaces (BFS/DFS).
    Deep(DeepArgs),
    /// Lista los enlaces de una página (mapa ligero, sin contenido).
    Map {
        /// URL a mapear.
        url: String,
        /// Modo de descarga.
        #[arg(long, value_enum, default_value_t = Mode::Auto)]
        mode: Mode,
        /// Proxy de salida.
        #[arg(long)]
        proxy: Option<String>,
        /// Ignora errores de certificado TLS.
        #[arg(long)]
        insecure: bool,
    },
    /// Lanza el servidor API.
    Serve {
        /// Puerto de escucha.
        #[arg(long, default_value_t = 8080)]
        port: u16,
        /// Dirección de escucha.
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        /// Activa el modo stealth en los crawls del servidor.
        #[arg(long)]
        stealth: bool,
    },
    /// Muestra la configuración por defecto en JSON.
    Config,
}

#[derive(clap::Args)]
struct CrawlArgs {
    /// URL a procesar.
    url: String,
    /// Consulta para el filtrado por relevancia (BM25).
    #[arg(long)]
    query: Option<String>,
    /// Procesa un fichero HTML local en lugar de descargar la URL.
    #[arg(long)]
    html_file: Option<String>,
    /// Modo de descarga: fast (HTTP), browser, o auto (por defecto).
    #[arg(long, value_enum, default_value_t = Mode::Auto)]
    mode: Mode,
    /// Imprime `fit_markdown` en lugar del markdown completo.
    #[arg(long)]
    fit: bool,
    /// Imprime el resultado completo como JSON.
    #[arg(long)]
    json: bool,
    /// Directorio de caché en disco (RAM + sled); reutiliza páginas ya vistas.
    #[arg(long)]
    cache: Option<String>,
    /// Activa el modo stealth (rotación de fingerprint + comportamiento humano).
    #[arg(long)]
    stealth: bool,
    /// Proxy de salida (`host:puerto` o `esquema://host:puerto`).
    #[arg(long)]
    proxy: Option<String>,
    /// Ignora errores de certificado TLS (inseguro; proxies interceptores).
    #[arg(long)]
    insecure: bool,
    /// Extrae un campo por CSS: `nombre=selector` (repetible).
    #[arg(long = "extract-css", value_name = "NOMBRE=SEL")]
    extract_css: Vec<String>,
    /// Extrae el contenido principal por densidad semántica.
    #[arg(long)]
    extract_semantic: bool,
    /// Extrae los objetos JSON-LD / schema.org de la página.
    #[arg(long)]
    jsonld: bool,
}

#[derive(clap::Args)]
struct DeepArgs {
    /// URL semilla desde la que arranca el recorrido.
    url: String,
    /// Consulta para el filtrado por relevancia (BM25).
    #[arg(long)]
    query: Option<String>,
    /// Estrategia de recorrido.
    #[arg(long, value_enum, default_value_t = Strategy::Bfs)]
    strategy: Strategy,
    /// Máximo de páginas a visitar.
    #[arg(long, default_value_t = 25)]
    max_pages: usize,
    /// Profundidad máxima (0 = sólo la semilla).
    #[arg(long, default_value_t = 2)]
    max_depth: usize,
    /// Descargas simultáneas.
    #[arg(long, default_value_t = 4)]
    concurrency: usize,
    /// Modo de descarga: fast (HTTP), browser, o auto (por defecto).
    #[arg(long, value_enum, default_value_t = Mode::Auto)]
    mode: Mode,
    /// Seguir también enlaces a otros dominios.
    #[arg(long)]
    cross_domain: bool,
    /// Directorio de caché en disco (RAM + sled); reutiliza páginas ya vistas.
    #[arg(long)]
    cache: Option<String>,
    /// Activa el modo stealth (rotación de fingerprint + comportamiento humano).
    #[arg(long)]
    stealth: bool,
    /// Proxy de salida (`host:puerto` o `esquema://host:puerto`).
    #[arg(long)]
    proxy: Option<String>,
    /// Ignora errores de certificado TLS (inseguro; proxies interceptores).
    #[arg(long)]
    insecure: bool,
    /// Imprime el informe completo como JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Strategy {
    Bfs,
    Dfs,
}

impl From<Strategy> for DeepStrategy {
    fn from(s: Strategy) -> Self {
        match s {
            Strategy::Bfs => DeepStrategy::Bfs,
            Strategy::Dfs => DeepStrategy::Dfs,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum Mode {
    /// HTTP puro, rápido, sin navegador.
    Fast,
    /// Navegador headless + stealth.
    Browser,
    /// HTTP y escala a navegador ante 403/challenge (por defecto).
    Auto,
}

impl From<Mode> for FetchMode {
    fn from(m: Mode) -> Self {
        match m {
            Mode::Fast => FetchMode::Fast,
            Mode::Browser => FetchMode::Browser,
            Mode::Auto => FetchMode::Auto,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Crawl(args) => run_crawl(args).await,
        Command::Deep(args) => run_deep(args).await,
        Command::Map {
            url,
            mode,
            proxy,
            insecure,
        } => run_map(url, mode, proxy, insecure).await,
        Command::Serve {
            port,
            host,
            stealth,
        } => run_serve(host, port, stealth).await,
        Command::Config => {
            let json = serde_json::to_string_pretty(&CrawlConfig::default())?;
            println!("{json}");
            Ok(())
        }
    }
}

/// Abre la caché de resultados en `dir`, si se indicó.
fn open_cache(dir: &Option<String>) -> Result<Option<ResultCache>> {
    match dir {
        Some(path) => Ok(Some(ResultCache::open(path, 1024)?)),
        None => Ok(None),
    }
}

/// Construye una estrategia de extracción a partir de las opciones de la CLI.
fn build_extraction(css: &[String], semantic: bool) -> Result<Option<Arc<dyn ExtractionStrategy>>> {
    if !css.is_empty() {
        let mut fields = Vec::with_capacity(css.len());
        for spec in css {
            let (name, selector) = spec
                .split_once('=')
                .ok_or_else(|| anyhow::anyhow!("--extract-css espera `nombre=selector`: {spec}"))?;
            let (name, selector) = (name.trim(), selector.trim());
            // Sintaxis de atributo: `selector::attr(src)`.
            if let Some((sel, rest)) = selector.split_once("::attr(") {
                let attr = rest.strip_suffix(')').ok_or_else(|| {
                    anyhow::anyhow!("sintaxis inválida, se esperaba `::attr(nombre)`: {selector}")
                })?;
                fields.push(FieldSpec::attr(name, sel.trim(), attr.trim()));
            } else {
                fields.push(FieldSpec::text(name, selector));
            }
        }
        Ok(Some(Arc::new(CssSelectorStrategy::new(fields))))
    } else if semantic {
        Ok(Some(Arc::new(SemanticDensityStrategy::new())))
    } else {
        Ok(None)
    }
}

/// Aplica una estrategia de extracción a un crawler, si la hay.
fn apply_extraction(crawler: Crawler, extraction: Option<Arc<dyn ExtractionStrategy>>) -> Crawler {
    match extraction {
        Some(strategy) => crawler.with_extraction(strategy),
        None => crawler,
    }
}

/// Ejecuta `f` con un crawler de navegador (caché y stealth opcionales),
/// cerrando Chromium de forma ordenada al terminar (aunque `f` devuelva error).
#[allow(clippy::too_many_arguments)]
async fn with_browser<F, Fut, T>(
    timeout_ms: u64,
    cache: Option<ResultCache>,
    stealth: bool,
    proxy: Option<String>,
    insecure: bool,
    extraction: Option<Arc<dyn ExtractionStrategy>>,
    f: F,
) -> Result<T>
where
    F: FnOnce(Crawler) -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut fetcher = BrowserFetcher::new().with_timeout(Duration::from_millis(timeout_ms));
    if stealth {
        fetcher = fetcher.with_stealth(Arc::new(StealthEngine::new(StealthConfig::default())));
    }
    if let Some(proxy) = proxy {
        fetcher = fetcher.with_proxy(proxy);
    }
    if insecure {
        fetcher = fetcher.with_insecure(true);
    }
    let browser = Arc::new(fetcher);

    // Fetcher HTTP rápido (para modos fast/auto). El navegador es perezoso:
    // en fast/auto-sin-challenge Chromium nunca se lanza.
    let http = if insecure {
        HttpFetcher::insecure()
    } else {
        HttpFetcher::new()
    };

    // Auto-login (opcional): comparte una celda de sesión entre marchas.
    let auto = auto_login_setup();
    // La marcha larga: la de auto-login si hay; si no, Playwright suelto o el
    // navegador propio.
    let long_gear: Arc<dyn crawl4rs_core::Fetcher> = match &auto {
        Some((_, _, pf)) => pf.clone(),
        None => {
            playwright_gear().unwrap_or_else(|| browser.clone() as Arc<dyn crawl4rs_core::Fetcher>)
        }
    };
    let mut crawler = Crawler::new(browser.clone()).with_browser_fetcher(long_gear);
    if let Ok(mut http) = http {
        if let Some((cell, _, _)) = &auto {
            http = http.with_session_cell(cell.clone());
        }
        crawler = crawler.with_http_fetcher(Arc::new(http));
    }
    if let Some((cell, auth, _)) = auto {
        crawler = crawler.with_auto_login(auth, cell);
    }
    if let Some(cache) = cache.clone() {
        crawler = crawler.with_cache(cache);
    }
    crawler = apply_extraction(crawler, extraction);
    let out = f(crawler).await;
    if let Some(cache) = cache {
        cache.flush();
    }
    if let Ok(browser) = Arc::try_unwrap(browser) {
        browser.shutdown().await;
    }
    out
}

/// Marcha larga por Playwright, si `CRAWL4RS_PLAYWRIGHT_URL` está definida.
/// El puente es HTTP fino (el MCP se reserva para la capa de agente). Sin la
/// variable, se usa el navegador propio (degradación honesta, additivo).
#[cfg(feature = "playwright")]
fn playwright_gear() -> Option<Arc<dyn crawl4rs_core::Fetcher>> {
    let url = std::env::var("CRAWL4RS_PLAYWRIGHT_URL").ok()?;
    match crawl4rs_core::PlaywrightFetcher::new(&url) {
        Ok(mut f) => {
            if let Some(pasos) = interact_pasos() {
                f = f.with_interact(pasos);
            }
            eprintln!("Marcha larga: Playwright en {url}");
            Some(Arc::new(f))
        }
        Err(e) => {
            eprintln!("AVISO: Playwright no disponible ({e}); uso el navegador propio.");
            None
        }
    }
}

/// Guion de interacción (scroll/click/…) desde `CRAWL4RS_INTERACT` (ruta a un
/// JSON con la lista de pasos). El wrapper lo ejecuta antes de leer el DOM.
#[cfg(feature = "playwright")]
fn interact_pasos() -> Option<serde_json::Value> {
    let path = std::env::var("CRAWL4RS_INTERACT").ok()?;
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| eprintln!("AVISO: no pude leer CRAWL4RS_INTERACT ({path}): {e}"))
        .ok()?;
    serde_json::from_str(&contents)
        .map_err(|e| eprintln!("AVISO: guion de interacción inválido: {e}"))
        .ok()
}

#[cfg(not(feature = "playwright"))]
fn playwright_gear() -> Option<Arc<dyn crawl4rs_core::Fetcher>> {
    None
}

/// Auto-login: celda de sesión compartida + autenticador + marcha larga con
/// esa celda. Requiere `CRAWL4RS_PLAYWRIGHT_URL` **y** `CRAWL4RS_LOGIN` (ruta a
/// un JSON `{ "url": "…", "pasos": [ … ] }`). Sin ellas, `None` (additivo).
type AutoLogin = (
    crawl4rs_core::session::SessionCell,
    Arc<dyn crawl4rs_core::session::Authenticator>,
    Arc<dyn crawl4rs_core::Fetcher>,
);

#[cfg(feature = "playwright")]
fn auto_login_setup() -> Option<AutoLogin> {
    let endpoint = std::env::var("CRAWL4RS_PLAYWRIGHT_URL").ok()?;
    let recipe_path = std::env::var("CRAWL4RS_LOGIN").ok()?;
    let contents = std::fs::read_to_string(&recipe_path)
        .map_err(|e| eprintln!("AVISO: no pude leer CRAWL4RS_LOGIN ({recipe_path}): {e}"))
        .ok()?;
    let v: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| eprintln!("AVISO: receta de login inválida: {e}"))
        .ok()?;
    let login_url = v.get("url").and_then(|x| x.as_str())?.to_string();
    let pasos = v
        .get("pasos")
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]));

    let cell = crawl4rs_core::session::empty_session_cell();
    let mut pf = crawl4rs_core::PlaywrightFetcher::new(&endpoint)
        .ok()?
        .with_session_cell(cell.clone());
    if let Some(guion) = interact_pasos() {
        pf = pf.with_interact(guion);
    }
    let auth: Arc<dyn crawl4rs_core::session::Authenticator> =
        Arc::new(pf.authenticator(login_url, pasos));
    eprintln!("Auto-login: receta {recipe_path} vía {endpoint}");
    Some((cell, auth, Arc::new(pf) as Arc<dyn crawl4rs_core::Fetcher>))
}

#[cfg(not(feature = "playwright"))]
fn auto_login_setup() -> Option<AutoLogin> {
    None
}

async fn run_serve(host: String, port: u16, stealth: bool) -> Result<()> {
    use crawl4rs_server::AuthConfig;
    use std::net::SocketAddr;

    // LA LEY DE LA FRONTERA: la auth protege una frontera, no un ritual.
    //   · secreto explícito              → auth ACTIVA (decisión del operador).
    //   · CRAWL4RS_AUTH=abierta          → auth ABIERTA declarada (la frontera vive en otra
    //     capa: p.ej. Docker publicando solo a 127.0.0.1 del host — el contenedor bindea
    //     0.0.0.0 por necesidad del port-mapping y no puede ver la frontera real).
    //   · loopback sin secreto           → auth ABIERTA (no hay frontera que proteger).
    //   · público sin secreto            → NEGARSE A ARRANCAR (fail-closed: jamás el
    //     default forjable de antes, jamás un secreto generado en silencio).
    let es_loopback = matches!(host.as_str(), "127.0.0.1" | "::1" | "localhost");
    let abierta_declarada = std::env::var("CRAWL4RS_AUTH")
        .map(|v| v.trim().eq_ignore_ascii_case("abierta"))
        .unwrap_or(false);
    let secret = std::env::var("CRAWL4RS_JWT_SECRET")
        .ok()
        .filter(|s| !s.trim().is_empty());
    let mut auth = match (secret, abierta_declarada, es_loopback) {
        (Some(secret), _, _) => {
            eprintln!("Auth JWT activa (CRAWL4RS_JWT_SECRET explícito).");
            AuthConfig::new(secret)
        }
        (None, true, _) => {
            eprintln!(
                "Auth ABIERTA declarada (CRAWL4RS_AUTH=abierta): la frontera vive en otra capa."
            );
            AuthConfig::abierta()
        }
        (None, false, true) => {
            eprintln!("Loopback sin secreto → auth ABIERTA (no hay frontera que proteger).");
            AuthConfig::abierta()
        }
        (None, false, false) => anyhow::bail!(
            "bindear a {host} expone el servidor más allá de loopback: define CRAWL4RS_JWT_SECRET \
             (o declara CRAWL4RS_AUTH=abierta si la frontera vive en otra capa)"
        ),
    };
    // API key PRESENTE pero vacía = NO configurada (una variable vacía blindaba
    // /auth/token con clave "" — cazado en producción).
    match std::env::var("CRAWL4RS_API_KEY")
        .ok()
        .filter(|k| !k.trim().is_empty())
    {
        Some(key) => {
            auth = auth.with_api_key(key);
            eprintln!("Emisión de tokens protegida por CRAWL4RS_API_KEY.");
        }
        None => eprintln!("Emisión de tokens abierta (define CRAWL4RS_API_KEY para restringirla)."),
    }

    let mut fetcher = BrowserFetcher::new();
    if stealth {
        fetcher = fetcher.with_stealth(Arc::new(StealthEngine::new(StealthConfig::default())));
    }
    let browser = Arc::new(fetcher);
    // El servidor sirve todos los modos: HTTP rápido + marcha larga. `mode` de
    // cada `POST /crawl` decide cuál se usa (auto por defecto). La marcha larga
    // es Playwright si CRAWL4RS_PLAYWRIGHT_URL está definida; si no, el navegador propio.
    let auto = auto_login_setup();
    let long_gear: Arc<dyn crawl4rs_core::Fetcher> = match &auto {
        Some((_, _, pf)) => pf.clone(),
        None => {
            playwright_gear().unwrap_or_else(|| browser.clone() as Arc<dyn crawl4rs_core::Fetcher>)
        }
    };
    let mut crawler = Crawler::new(browser.clone()).with_browser_fetcher(long_gear);
    if let Ok(mut http) = HttpFetcher::new() {
        if let Some((cell, _, _)) = &auto {
            http = http.with_session_cell(cell.clone());
        }
        crawler = crawler.with_http_fetcher(Arc::new(http));
    }
    if let Some((cell, auth_login, _)) = auto {
        crawler = crawler.with_auto_login(auth_login, cell);
    }

    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .map_err(|e| anyhow::anyhow!("dirección inválida {host}:{port}: {e}"))?;
    eprintln!("Servidor en http://{addr}  ·  dashboard en http://{addr}/dashboard");
    crawl4rs_server::serve(addr, crawler, auth).await?;
    Ok(())
}

async fn run_map(url: String, mode: Mode, proxy: Option<String>, insecure: bool) -> Result<()> {
    let config = CrawlConfig {
        mode: mode.into(),
        ..Default::default()
    };
    let u = url.clone();
    let links = with_browser(
        config.timeout_ms,
        None,
        false,
        proxy,
        insecure,
        None,
        |crawler| async move { Ok(crawler.map(&u, &config).await?) },
    )
    .await?;
    println!("{}", serde_json::to_string_pretty(&links)?);
    Ok(())
}

async fn run_crawl(args: CrawlArgs) -> Result<()> {
    let config = CrawlConfig {
        mode: args.mode.into(),
        extract_jsonld: args.jsonld,
        ..CrawlConfig::default().with_query(args.query.clone())
    };

    let extraction = build_extraction(&args.extract_css, args.extract_semantic)?;

    let result = match &args.html_file {
        Some(path) => {
            let html = std::fs::read_to_string(path)?;
            let crawler =
                apply_extraction(Crawler::new(Arc::new(StaticFetcher::new(html))), extraction);
            crawler.crawl(&args.url, &config).await?
        }
        None => {
            let url = args.url.clone();
            let cache = open_cache(&args.cache)?;
            with_browser(
                config.timeout_ms,
                cache,
                args.stealth,
                args.proxy.clone(),
                args.insecure,
                extraction,
                |crawler| async move { Ok(crawler.crawl(&url, &config).await?) },
            )
            .await?
        }
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else if args.fit {
        println!("{}", result.fit_markdown);
    } else {
        println!("{}", result.markdown);
    }
    Ok(())
}

async fn run_deep(args: DeepArgs) -> Result<()> {
    let config = CrawlConfig {
        query: args.query.clone(),
        max_pages: args.max_pages,
        max_depth: args.max_depth,
        deep_strategy: args.strategy.into(),
        same_domain: !args.cross_domain,
        concurrency: args.concurrency,
        stealth: args.stealth,
        mode: args.mode.into(),
        ..Default::default()
    };

    let url = args.url.clone();
    let cache = open_cache(&args.cache)?;
    let report = with_browser(
        config.timeout_ms,
        cache,
        args.stealth,
        args.proxy.clone(),
        args.insecure,
        None,
        |crawler| async move { Ok(crawler.crawl_deep(&url, &config).await?) },
    )
    .await?;

    if args.json {
        let json = serde_json::json!({
            "pages": report.pages,
            "errors": report.errors
                .iter()
                .map(|(u, e)| serde_json::json!({ "url": u, "error": e }))
                .collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        eprintln!(
            "Recorridas {} páginas ({} errores).\n",
            report.pages.len(),
            report.errors.len()
        );
        for page in &report.pages {
            println!("# {}\n", page.url);
            println!("{}\n", page.fit_markdown);
            println!("---\n");
        }
    }
    Ok(())
}
