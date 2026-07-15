//! Fetcher "marcha larga": delega en el **wrapper HTTP de Playwright**.
//!
//! El puente Crawl4RS ↔ Playwright es una costura máquina-a-máquina y
//! determinista, así que el transporte es **HTTP fino** (no el MCP, que se
//! reserva para la capa de agente). Un wrapper Node minúsculo envuelve
//! Playwright-librería y expone el contrato `contrato-puente-v1`:
//!
//! ```text
//! POST /abrir  { url }  ->  { html, final_url, status }   (+ fallo? en errores)
//! ```
//!
//! Este `Fetcher` implementa solo el subconjunto `ahora` (`abrir(url) → html`);
//! el wrapper declara el resto (login, interceptar, emular…) como reservado.
//! Se conecta como *marcha larga* con [`crate::crawler::Crawler::with_browser_fetcher`].
//!
//! Ver `docs/contrato-puente-prisma.md`.

use std::sync::{Arc, RwLock};
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::fetch::{FetchedPage, Fetcher};
use crate::session::{Authenticator, Session, SessionCell};

/// Fetcher que abre la página a través del wrapper HTTP de Playwright.
#[derive(Clone)]
pub struct PlaywrightFetcher {
    client: reqwest::Client,
    endpoint: String,
    /// Sesión a reutilizar (`storageState`) en cada `POST /abrir`, si la hay.
    /// Celda compartida para que el re-login la refresque en caliente.
    session: Option<SessionCell>,
    /// Guion de interacción (scroll/click/…) aplicado antes de leer el DOM.
    interact: Option<serde_json::Value>,
    /// Config de interceptación de la API interna (`true` o `{ contiene:[…] }`).
    intercept: Option<serde_json::Value>,
    /// Modo stealth (oculta señales de automatización).
    stealth: bool,
    /// Proxy de salida (`{ server, username?, password? }`).
    proxy: Option<serde_json::Value>,
    /// Emulación (`{ locale?, timezone?, geo?, movil? }`).
    emulate: Option<serde_json::Value>,
}

fn es_false(b: &bool) -> bool {
    !*b
}

/// Cuerpo de `POST /abrir`.
#[derive(Serialize)]
struct AbrirReq<'a> {
    url: &'a str,
    /// `storageState` para abrir ya autenticado (reservado → ahora).
    #[serde(skip_serializing_if = "Option::is_none")]
    sesion: Option<&'a serde_json::Value>,
    /// Guion de interacción antes de leer el DOM (reservado → ahora).
    #[serde(skip_serializing_if = "Option::is_none")]
    interactuar: Option<&'a serde_json::Value>,
    /// Interceptar el JSON de la API interna (reservado → ahora).
    #[serde(skip_serializing_if = "Option::is_none")]
    interceptar: Option<&'a serde_json::Value>,
    /// Modo stealth (reservado → ahora).
    #[serde(skip_serializing_if = "es_false")]
    stealth: bool,
    /// Proxy de salida (reservado → ahora).
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<&'a serde_json::Value>,
    /// Emulación (reservado → ahora).
    #[serde(skip_serializing_if = "Option::is_none")]
    emular: Option<&'a serde_json::Value>,
}

/// Cuerpo de `POST /login`.
#[derive(Serialize)]
struct LoginReq<'a> {
    url: &'a str,
    pasos: &'a serde_json::Value,
    #[serde(skip_serializing_if = "es_false")]
    stealth: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<&'a serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emular: Option<&'a serde_json::Value>,
}

/// Respuesta de `POST /login`.
#[derive(Deserialize)]
struct LoginResp {
    sesion: Option<serde_json::Value>,
    fallo: Option<Fallo>,
}

/// Fallo reportado por el wrapper (verdad_obligatoria: no se inventa, se dice).
#[derive(Deserialize)]
struct Fallo {
    tipo: String,
    motivo: String,
}

/// Respuesta de `POST /abrir` (subconjunto `ahora`).
#[derive(Deserialize)]
struct AbrirResp {
    html: Option<String>,
    final_url: Option<String>,
    status: Option<u16>,
    fallo: Option<Fallo>,
    #[serde(default)]
    intercepted: Vec<serde_json::Value>,
}

impl PlaywrightFetcher {
    /// Crea el fetcher apuntando al `endpoint` del wrapper (p. ej.
    /// `http://playwright:8100`). Timeout de 60 s (la marcha larga es lenta).
    pub fn new(endpoint: impl Into<String>) -> Result<Self> {
        Self::with_timeout(endpoint, Duration::from_secs(60))
    }

    /// Igual que [`Self::new`] con un timeout explícito.
    pub fn with_timeout(endpoint: impl Into<String>, timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| Error::fetch("<cliente playwright>", e))?;
        let endpoint = endpoint.into();
        let endpoint = endpoint.trim_end_matches('/').to_string();
        Ok(Self {
            client,
            endpoint,
            session: None,
            interact: None,
            intercept: None,
            stealth: false,
            proxy: None,
            emulate: None,
        })
    }

    /// Reutiliza una sesión fija: cada `POST /abrir` la incluye para abrir ya
    /// autenticado. Additivo: sin sesión, se comporta igual que antes.
    pub fn with_session(mut self, session: Session) -> Self {
        self.session = Some(Arc::new(RwLock::new(Some(session))));
        self
    }

    /// Configura un guion de interacción (scroll/click/fill/wait) que el wrapper
    /// ejecuta antes de leer el DOM — para scroll infinito, "cargar más", etc.
    pub fn with_interact(mut self, pasos: serde_json::Value) -> Self {
        self.interact = Some(pasos);
        self
    }

    /// Activa la interceptación de la API interna. `config` = `true` (todo el
    /// JSON) o `{ "contiene": ["/api/", "graphql"] }` (filtra por URL). El JSON
    /// capturado llega en [`FetchedPage::intercepted`] / [`CrawlResult`].
    pub fn with_intercept(mut self, config: serde_json::Value) -> Self {
        self.intercept = Some(config);
        self
    }

    /// Activa el modo stealth (oculta señales de automatización). Ayuda con
    /// anti-bot medio; no promete DataDome/Turnstile.
    pub fn with_stealth(mut self, on: bool) -> Self {
        self.stealth = on;
        self
    }

    /// Configura el proxy de salida de la marcha larga:
    /// `{ "server": "http://host:puerto", "username"?: …, "password"?: … }`.
    pub fn with_proxy(mut self, proxy: serde_json::Value) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Configura la emulación:
    /// `{ "locale"?, "timezone"?, "geo"?: {latitude,longitude}, "movil"?: bool }`.
    pub fn with_emulate(mut self, config: serde_json::Value) -> Self {
        self.emulate = Some(config);
        self
    }

    /// Comparte una celda de sesión (la que el re-login refresca en caliente).
    pub fn with_session_cell(mut self, cell: SessionCell) -> Self {
        self.session = Some(cell);
        self
    }

    /// Envuelve este fetcher en un [`Authenticator`] atado a una receta de login
    /// (`url` + `pasos`), para el lazo automático de re-login del crawler.
    pub fn authenticator(
        &self,
        url: impl Into<String>,
        pasos: serde_json::Value,
    ) -> PlaywrightAuth {
        PlaywrightAuth {
            fetcher: self.clone(),
            url: url.into(),
            pasos,
        }
    }

    /// Hace login en `url` ejecutando `pasos` (guion de acciones que el wrapper
    /// entiende: `fill`/`click`/`wait`) y devuelve la [`Session`] capturada
    /// (`storageState`). El fallo se propaga; no se inventa una sesión.
    pub async fn login(&self, url: &str, pasos: serde_json::Value) -> Result<Session> {
        let body = serde_json::to_string(&LoginReq {
            url,
            pasos: &pasos,
            stealth: self.stealth,
            proxy: self.proxy.as_ref(),
            emular: self.emulate.as_ref(),
        })
        .map_err(|e| Error::Browser(format!("no se pudo serializar el login: {e}")))?;
        let resp = self
            .client
            .post(format!("{}/login", self.endpoint))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::fetch(url, e))?;

        let http_status = resp.status();
        let text = resp.text().await.map_err(|e| Error::fetch(url, e))?;
        if !http_status.is_success() {
            let snippet: String = text.chars().take(200).collect();
            return Err(Error::Browser(format!(
                "wrapper Playwright respondió {http_status} al login: {snippet}"
            )));
        }
        let parsed: LoginResp = serde_json::from_str(&text)
            .map_err(|e| Error::Browser(format!("respuesta de login ilegible: {e}")))?;
        if let Some(f) = parsed.fallo {
            return Err(Error::Browser(format!(
                "login falló en {url}: {} — {}",
                f.tipo, f.motivo
            )));
        }
        let sesion = parsed
            .sesion
            .ok_or_else(|| Error::Browser("el login no devolvió sesión".into()))?;
        Ok(Session::from_storage_state(sesion))
    }
}

#[async_trait]
impl Fetcher for PlaywrightFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        // Se clona el storageState (el guard no se mantiene a través del await).
        let sesion_owned: Option<serde_json::Value> = self.session.as_ref().and_then(|cell| {
            cell.read()
                .ok()
                .and_then(|g| g.as_ref().map(|s| s.storage_state.clone()))
        });
        let body = serde_json::to_string(&AbrirReq {
            url,
            sesion: sesion_owned.as_ref(),
            interactuar: self.interact.as_ref(),
            interceptar: self.intercept.as_ref(),
            stealth: self.stealth,
            proxy: self.proxy.as_ref(),
            emular: self.emulate.as_ref(),
        })
        .map_err(|e| Error::Browser(format!("no se pudo serializar la petición: {e}")))?;

        let resp = self
            .client
            .post(format!("{}/abrir", self.endpoint))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::fetch(url, e))?;

        let http_status = resp.status();
        let text = resp.text().await.map_err(|e| Error::fetch(url, e))?;

        if !http_status.is_success() {
            let snippet: String = text.chars().take(200).collect();
            return Err(Error::Browser(format!(
                "wrapper Playwright respondió {http_status}: {snippet}"
            )));
        }

        let parsed: AbrirResp = serde_json::from_str(&text)
            .map_err(|e| Error::Browser(format!("respuesta del wrapper ilegible: {e}")))?;

        // El wrapper reporta el fallo real; lo propagamos, no inventamos HTML.
        if let Some(f) = parsed.fallo {
            return Err(Error::Browser(format!(
                "Playwright no pudo abrir {url}: {} — {}",
                f.tipo, f.motivo
            )));
        }

        Ok(FetchedPage {
            url: parsed.final_url.unwrap_or_else(|| url.to_string()),
            status: parsed.status,
            html: parsed.html.unwrap_or_default(),
            content_type: Some("text/html".to_string()),
            bytes: None,
            intercepted: parsed.intercepted,
        })
    }
}

/// [`Authenticator`] que hace login por el wrapper con una receta fija.
#[derive(Clone)]
pub struct PlaywrightAuth {
    fetcher: PlaywrightFetcher,
    url: String,
    pasos: serde_json::Value,
}

#[async_trait]
impl Authenticator for PlaywrightAuth {
    async fn login(&self) -> Result<Session> {
        self.fetcher.login(&self.url, self.pasos.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    /// Servidor HTTP mínimo de un solo golpe que devuelve `body` fijo y captura
    /// la petición recibida (línea + cabeceras + cuerpo). Evita depender de un
    /// mock externo para probar la costura.
    fn servidor_captura(body: &'static str) -> (String, std::sync::mpsc::Receiver<String>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 8192];
                let n = stream.read(&mut buf).unwrap_or(0);
                let _ = tx.send(String::from_utf8_lossy(&buf[..n]).into_owned());
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        (format!("http://{addr}"), rx)
    }

    fn servidor_una_vez(body: &'static str) -> String {
        servidor_captura(body).0
    }

    #[tokio::test]
    async fn abrir_devuelve_html() {
        let endpoint =
            servidor_una_vez(r#"{"html":"<h1>hola</h1>","final_url":"https://x/y","status":200}"#);
        let f = PlaywrightFetcher::new(endpoint).unwrap();
        let page = f.fetch("https://x/").await.unwrap();
        assert_eq!(page.html, "<h1>hola</h1>");
        assert_eq!(page.status, Some(200));
        assert_eq!(page.url, "https://x/y");
    }

    #[tokio::test]
    async fn fallo_del_wrapper_se_propaga_no_se_inventa() {
        let endpoint =
            servidor_una_vez(r#"{"fallo":{"tipo":"reto_no_superado","motivo":"cloudflare"}}"#);
        let f = PlaywrightFetcher::new(endpoint).unwrap();
        let err = f.fetch("https://x/").await.unwrap_err();
        assert!(err.to_string().contains("reto_no_superado"));
    }

    #[tokio::test]
    async fn abrir_incluye_la_sesion_en_el_cuerpo() {
        let (endpoint, rx) = servidor_captura(r#"{"html":"<b>ok</b>","status":200}"#);
        let sesion = crate::session::Session::from_storage_state(
            serde_json::json!({ "cookies": [{ "name": "sid", "value": "abc" }] }),
        );
        let f = PlaywrightFetcher::new(endpoint)
            .unwrap()
            .with_session(sesion);
        f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("/abrir"), "va a /abrir");
        assert!(req.contains("\"sesion\""), "el cuerpo lleva la sesión");
        assert!(req.contains("sid"), "lleva la cookie de la sesión");
    }

    #[tokio::test]
    async fn login_captura_la_sesion() {
        let (endpoint, rx) =
            servidor_captura(r#"{"sesion":{"cookies":[{"name":"sid","value":"xyz"}]}}"#);
        let f = PlaywrightFetcher::new(endpoint).unwrap();
        let pasos = serde_json::json!([
            { "tipo": "fill", "selector": "#user", "valor": "yo" },
            { "tipo": "click", "selector": "#entrar" }
        ]);
        let sesion = f.login("https://x/login", pasos).await.unwrap();
        assert_eq!(sesion.cookie_header().as_deref(), Some("sid=xyz"));
        let req = rx.recv().unwrap();
        assert!(req.contains("/login"), "va a /login");
        assert!(req.contains("pasos"), "el cuerpo lleva los pasos");
    }

    #[tokio::test]
    async fn abrir_incluye_el_guion_de_interaccion() {
        let (endpoint, rx) = servidor_captura(r#"{"html":"<b>ok</b>","status":200}"#);
        let f = PlaywrightFetcher::new(endpoint)
            .unwrap()
            .with_interact(serde_json::json!([{ "tipo": "scroll", "veces": 3 }]));
        f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("\"interactuar\""), "el cuerpo lleva el guion");
        assert!(req.contains("scroll"), "lleva el paso scroll");
    }

    #[tokio::test]
    async fn interceptar_pide_y_devuelve_el_json_capturado() {
        let (endpoint, rx) = servidor_captura(
            r#"{"html":"<b>ok</b>","status":200,"intercepted":[{"url":"/api/precios","json":{"precio":9}}]}"#,
        );
        let f = PlaywrightFetcher::new(endpoint)
            .unwrap()
            .with_intercept(serde_json::json!(true));
        let page = f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(
            req.contains("\"interceptar\""),
            "el cuerpo pide interceptar"
        );
        assert_eq!(page.intercepted.len(), 1, "llega el JSON capturado");
        assert_eq!(page.intercepted[0]["json"]["precio"], 9);
    }

    #[tokio::test]
    async fn abrir_incluye_stealth_y_proxy() {
        let (endpoint, rx) = servidor_captura(r#"{"html":"<b>ok</b>","status":200}"#);
        let f = PlaywrightFetcher::new(endpoint)
            .unwrap()
            .with_stealth(true)
            .with_proxy(serde_json::json!({ "server": "http://p:8080" }));
        f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("\"stealth\":true"), "el cuerpo pide stealth");
        assert!(req.contains("http://p:8080"), "lleva el server del proxy");
    }

    #[tokio::test]
    async fn sin_stealth_ni_proxy_no_van_en_el_cuerpo() {
        let (endpoint, rx) = servidor_captura(r#"{"html":"<b>ok</b>","status":200}"#);
        let f = PlaywrightFetcher::new(endpoint).unwrap();
        f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(!req.contains("stealth"), "sin stealth no se envía");
        assert!(!req.contains("proxy"), "sin proxy no se envía");
        assert!(!req.contains("emular"), "sin emular no se envía");
    }

    #[tokio::test]
    async fn abrir_incluye_la_emulacion_en_el_cuerpo() {
        let (endpoint, rx) = servidor_captura(r#"{"html":"<b>ok</b>","status":200}"#);
        let f = PlaywrightFetcher::new(endpoint)
            .unwrap()
            .with_emulate(serde_json::json!({ "locale": "fr-FR", "movil": true }));
        f.fetch("https://x/").await.unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("\"emular\""), "el cuerpo lleva emular");
        assert!(req.contains("fr-FR"), "lleva el locale");
    }
}
