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
    FieldSpec, ResultCache, SemanticDensityStrategy, StaticFetcher, StealthConfig, StealthEngine,
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
            fields.push(FieldSpec::text(name.trim(), selector.trim()));
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
    let fetcher = Arc::new(fetcher);
    let mut crawler = Crawler::new(fetcher.clone());
    if let Some(cache) = cache.clone() {
        crawler = crawler.with_cache(cache);
    }
    crawler = apply_extraction(crawler, extraction);
    let out = f(crawler).await;
    if let Some(cache) = cache {
        cache.flush();
    }
    if let Ok(fetcher) = Arc::try_unwrap(fetcher) {
        fetcher.shutdown().await;
    }
    out
}

async fn run_serve(host: String, port: u16, stealth: bool) -> Result<()> {
    use crawl4rs_server::AuthConfig;
    use std::net::SocketAddr;

    let secret = std::env::var("CRAWL4RS_JWT_SECRET")
        .unwrap_or_else(|_| "crawl4rs-dev-secret-cambia-esto".to_string());
    let mut auth = AuthConfig::new(secret);
    if let Ok(key) = std::env::var("CRAWL4RS_API_KEY") {
        auth = auth.with_api_key(key);
        eprintln!("Emisión de tokens protegida por CRAWL4RS_API_KEY.");
    } else {
        eprintln!("AVISO: emisión de tokens abierta (define CRAWL4RS_API_KEY para restringirla).");
    }

    let mut fetcher = BrowserFetcher::new();
    if stealth {
        fetcher = fetcher.with_stealth(Arc::new(StealthEngine::new(StealthConfig::default())));
    }
    let crawler = Crawler::new(Arc::new(fetcher));

    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .map_err(|e| anyhow::anyhow!("dirección inválida {host}:{port}: {e}"))?;
    eprintln!("Servidor en http://{addr}  ·  dashboard en http://{addr}/dashboard");
    crawl4rs_server::serve(addr, crawler, auth).await?;
    Ok(())
}

async fn run_crawl(args: CrawlArgs) -> Result<()> {
    let config = CrawlConfig::default().with_query(args.query.clone());

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
