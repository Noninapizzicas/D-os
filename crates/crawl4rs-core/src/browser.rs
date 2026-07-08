//! Fetcher real vía Chromium/CDP (`chromiumoxide`) — Fase 1 de la hoja de ruta.
//!
//! - [`BrowserPool`]: una instancia de Chromium reutilizada, con un límite de
//!   pestañas concurrentes (semáforo).
//! - [`BrowserFetcher`]: implementa [`Fetcher`] lanzando el pool de forma
//!   perezosa en el primer uso.
//! - [`SessionManager`]: persiste cookies y `localStorage` por perfil en
//!   disco (JSON).

use std::path::{Path, PathBuf};
use std::time::Duration;

use async_trait::async_trait;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use chromiumoxide::page::Page;
use futures::StreamExt;
use tokio::sync::{OnceCell, Semaphore};
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};

use crate::error::{Error, Result};
use crate::fetch::{FetchedPage, Fetcher};

/// Rutas candidatas donde buscar el ejecutable de Chromium si no se indica.
const CHROME_CANDIDATES: &[&str] = &[
    "/opt/pw-browsers/chromium",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
    "/usr/bin/google-chrome",
    "/usr/bin/google-chrome-stable",
];

/// Configuración del pool de navegador.
#[derive(Debug, Clone)]
pub struct BrowserPoolConfig {
    /// Ruta al ejecutable de Chromium. `None` → se busca en
    /// `$CRAWL4RS_CHROME`, rutas conocidas y la detección de `chromiumoxide`.
    pub executable: Option<PathBuf>,
    /// Ejecutar sin interfaz gráfica.
    pub headless: bool,
    /// Añadir `--no-sandbox` (necesario en contenedores que corren como root).
    pub no_sandbox: bool,
    /// Máximo de pestañas abiertas a la vez.
    pub max_concurrent_pages: usize,
    /// Tamaño de la ventana (ancho, alto).
    pub window: (u32, u32),
    /// Directorio de perfil (`--user-data-dir`). `None` → se crea uno
    /// temporal único, lo que permite varios pools simultáneos sin que
    /// compitan por el perfil por defecto de Chromium.
    pub user_data_dir: Option<PathBuf>,
}

impl Default for BrowserPoolConfig {
    fn default() -> Self {
        Self {
            executable: None,
            headless: true,
            no_sandbox: true,
            max_concurrent_pages: 4,
            window: (1280, 800),
            user_data_dir: None,
        }
    }
}

/// Crea un directorio de perfil temporal único para esta instancia.
fn unique_profile_dir() -> std::io::Result<PathBuf> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "crawl4rs-profile-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed),
    ));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Localiza el ejecutable de Chromium a usar.
pub fn detect_chrome_executable() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("CRAWL4RS_CHROME") {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
        warn!(ruta = %p.display(), "$CRAWL4RS_CHROME no existe; se ignora");
    }
    CHROME_CANDIDATES
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
}

/// Una instancia de Chromium compartida con límite de pestañas concurrentes.
pub struct BrowserPool {
    browser: Browser,
    handler_task: JoinHandle<()>,
    pages: Semaphore,
}

impl BrowserPool {
    /// Lanza Chromium según la configuración dada.
    pub async fn launch(config: &BrowserPoolConfig) -> Result<Self> {
        let mut builder = BrowserConfig::builder()
            .window_size(config.window.0, config.window.1)
            .disable_default_args()
            .args(DEFAULT_ARGS.iter().copied());

        if !config.headless {
            builder = builder.with_head();
        }
        if config.no_sandbox {
            builder = builder.no_sandbox();
        }
        let executable = config.executable.clone().or_else(detect_chrome_executable);
        if let Some(exe) = &executable {
            builder = builder.chrome_executable(exe);
        }

        let profile_dir = match &config.user_data_dir {
            Some(dir) => dir.clone(),
            None => unique_profile_dir()
                .map_err(|e| Error::Browser(format!("no se pudo crear el perfil temporal: {e}")))?,
        };
        builder = builder.user_data_dir(&profile_dir);

        let browser_config = builder
            .build()
            .map_err(|e| Error::Browser(format!("configuración de navegador inválida: {e}")))?;

        info!(ejecutable = ?executable, "lanzando Chromium");
        let (browser, mut handler) = Browser::launch(browser_config)
            .await
            .map_err(|e| Error::Browser(format!("no se pudo lanzar Chromium: {e}")))?;

        // El handler debe sondearse continuamente para que avance el CDP.
        let handler_task = tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                if let Err(e) = event {
                    debug!(error = %e, "evento CDP con error");
                }
            }
        });

        Ok(Self {
            browser,
            handler_task,
            pages: Semaphore::new(config.max_concurrent_pages.max(1)),
        })
    }

    /// Navega a `url`, espera la carga y devuelve el HTML renderizado.
    ///
    /// El código de estado HTTP no se captura todavía (requiere escuchar
    /// eventos de red; Fase 4) — se devuelve `None`.
    pub async fn fetch_page(&self, url: &str, timeout: Duration) -> Result<FetchedPage> {
        let _permit = self
            .pages
            .acquire()
            .await
            .map_err(|_| Error::Browser("el pool de navegador está cerrado".into()))?;

        tokio::time::timeout(timeout, self.fetch_inner(url))
            .await
            .map_err(|_| {
                Error::Browser(format!(
                    "timeout tras {} ms cargando {url}",
                    timeout.as_millis()
                ))
            })?
    }

    async fn fetch_inner(&self, url: &str) -> Result<FetchedPage> {
        let page = self
            .browser
            .new_page(url)
            .await
            .map_err(|e| Error::fetch(url, e))?;

        // Espera al evento `load`; si la página ya cargó, vuelve al instante.
        // Un fallo aquí no es fatal: aún podemos leer el contenido actual.
        if let Err(e) = page.wait_for_navigation().await {
            debug!(error = %e, "wait_for_navigation falló; se continúa");
        }

        let html = page.content().await.map_err(|e| Error::fetch(url, e))?;
        let final_url = page
            .url()
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| url.to_string());

        if let Err(e) = page.close().await {
            debug!(error = %e, "no se pudo cerrar la pestaña");
        }

        Ok(FetchedPage {
            url: final_url,
            status: None,
            html,
        })
    }

    /// Abre una pestaña sin cerrarla, para uso avanzado (sesiones, JS).
    pub async fn new_page(&self, url: &str) -> Result<Page> {
        self.browser
            .new_page(url)
            .await
            .map_err(|e| Error::fetch(url, e))
    }

    /// Cierra el navegador y detiene el handler.
    pub async fn close(mut self) {
        if let Err(e) = self.browser.close().await {
            warn!(error = %e, "error al cerrar Chromium");
        }
        let _ = self.browser.wait().await;
        self.handler_task.abort();
    }
}

/// Argumentos de línea de comandos con los que se lanza Chromium.
///
/// Subconjunto conservador pensado para scraping en contenedores; la
/// rotación de fingerprint real llega con `crawl4rs-stealth` (Fase 5).
const DEFAULT_ARGS: &[&str] = &[
    "--disable-background-networking",
    "--disable-background-timer-throttling",
    "--disable-breakpad",
    "--disable-client-side-phishing-detection",
    "--disable-default-apps",
    "--disable-dev-shm-usage",
    "--disable-extensions",
    "--disable-features=TranslateUI",
    "--disable-hang-monitor",
    "--disable-ipc-flooding-protection",
    "--disable-popup-blocking",
    "--disable-prompt-on-repost",
    "--disable-renderer-backgrounding",
    "--disable-sync",
    "--metrics-recording-only",
    "--no-first-run",
    "--no-default-browser-check",
    "--mute-audio",
    "--hide-scrollbars",
];

/// [`Fetcher`] basado en navegador. Lanza el [`BrowserPool`] de forma
/// perezosa en la primera descarga y lo reutiliza después.
pub struct BrowserFetcher {
    config: BrowserPoolConfig,
    timeout: Duration,
    pool: OnceCell<BrowserPool>,
}

impl Default for BrowserFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserFetcher {
    /// Crea el fetcher con configuración por defecto (headless, 4 pestañas).
    pub fn new() -> Self {
        Self::with_config(BrowserPoolConfig::default())
    }

    /// Crea el fetcher con una configuración de pool concreta.
    pub fn with_config(config: BrowserPoolConfig) -> Self {
        Self {
            config,
            timeout: Duration::from_millis(30_000),
            pool: OnceCell::new(),
        }
    }

    /// Fija el tiempo máximo de carga por página.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    async fn pool(&self) -> Result<&BrowserPool> {
        self.pool
            .get_or_try_init(|| BrowserPool::launch(&self.config))
            .await
    }

    /// Cierra el navegador de forma ordenada, si llegó a lanzarse.
    pub async fn shutdown(self) {
        if let Some(pool) = self.pool.into_inner() {
            pool.close().await;
        }
    }
}

#[async_trait]
impl Fetcher for BrowserFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        self.pool().await?.fetch_page(url, self.timeout).await
    }
}

/// Persistencia de sesiones (cookies y `localStorage`) por perfil, en disco.
#[derive(Debug, Clone)]
pub struct SessionManager {
    dir: PathBuf,
}

impl SessionManager {
    /// Crea el gestor sobre un directorio (se crea si no existe).
    pub fn new(dir: impl AsRef<Path>) -> std::io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    fn cookies_path(&self, profile: &str) -> PathBuf {
        self.dir.join(format!("{profile}.cookies.json"))
    }

    fn storage_path(&self, profile: &str) -> PathBuf {
        self.dir.join(format!("{profile}.storage.json"))
    }

    /// Guarda las cookies de la página en el perfil dado.
    pub async fn save_cookies(&self, page: &Page, profile: &str) -> Result<()> {
        let cookies = page
            .get_cookies()
            .await
            .map_err(|e| Error::Browser(format!("no se pudieron leer cookies: {e}")))?;
        let json =
            serde_json::to_string_pretty(&cookies).map_err(|e| Error::Browser(e.to_string()))?;
        std::fs::write(self.cookies_path(profile), json)
            .map_err(|e| Error::Browser(e.to_string()))?;
        Ok(())
    }

    /// Restaura en la página las cookies guardadas del perfil dado.
    pub async fn restore_cookies(&self, page: &Page, profile: &str) -> Result<()> {
        let path = self.cookies_path(profile);
        if !path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(path).map_err(|e| Error::Browser(e.to_string()))?;
        // Las `Cookie` de CDP se re-leen como `CookieParam`: comparten los
        // campos relevantes y serde ignora los sobrantes (`size`, `session`…).
        let params: Vec<CookieParam> =
            serde_json::from_str(&json).map_err(|e| Error::Browser(e.to_string()))?;
        page.set_cookies(params)
            .await
            .map_err(|e| Error::Browser(format!("no se pudieron fijar cookies: {e}")))?;
        Ok(())
    }

    /// Guarda el `localStorage` del origen actual de la página.
    pub async fn save_local_storage(&self, page: &Page, profile: &str) -> Result<()> {
        let value: String = page
            .evaluate("JSON.stringify(Object.entries(localStorage))")
            .await
            .map_err(|e| Error::Browser(format!("no se pudo leer localStorage: {e}")))?
            .into_value()
            .map_err(|e| Error::Browser(e.to_string()))?;
        std::fs::write(self.storage_path(profile), value)
            .map_err(|e| Error::Browser(e.to_string()))?;
        Ok(())
    }

    /// Restaura el `localStorage` guardado en el origen actual de la página.
    pub async fn restore_local_storage(&self, page: &Page, profile: &str) -> Result<()> {
        let path = self.storage_path(profile);
        if !path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(path).map_err(|e| Error::Browser(e.to_string()))?;
        let entries: Vec<(String, String)> =
            serde_json::from_str(&json).map_err(|e| Error::Browser(e.to_string()))?;
        for (key, value) in entries {
            let script = format!(
                "localStorage.setItem({}, {})",
                serde_json::to_string(&key).unwrap(),
                serde_json::to_string(&value).unwrap(),
            );
            page.evaluate(script)
                .await
                .map_err(|e| Error::Browser(format!("no se pudo escribir localStorage: {e}")))?;
        }
        Ok(())
    }
}
