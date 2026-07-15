//! Fetcher HTTP rápido, sin navegador (feature `http`).
//!
//! Para páginas simples (la mayoría), abrir Chromium es un derroche de RAM y
//! latencia en un VPS pequeño. Este fetcher usa `reqwest`: una petición GET
//! con cabeceras realistas, redirecciones seguidas y descompresión. El
//! orquestador lo usa como camino por defecto y sólo escala a navegador
//! cuando hace falta (ver [`crate::config::FetchMode`]).

use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::fetch::{FetchedPage, Fetcher};
use crate::session::{Session, SessionCell};

const DEFAULT_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

/// Fetcher HTTP basado en `reqwest`.
#[derive(Clone)]
pub struct HttpFetcher {
    client: reqwest::Client,
    /// Sesión reutilizada (marcha corta): solo aporta las cookies. Celda
    /// compartida para que el re-login la refresque en caliente.
    session: Option<SessionCell>,
}

impl HttpFetcher {
    /// Crea el fetcher con valores por defecto (UA de navegador, timeout 30 s,
    /// hasta 10 redirecciones, gzip/brotli).
    pub fn new() -> Result<Self> {
        Self::build(false, std::time::Duration::from_secs(30))
    }

    /// Crea el fetcher permitiendo certificados TLS no válidos (inseguro; para
    /// proxies interceptores o staging con certs self-signed).
    pub fn insecure() -> Result<Self> {
        Self::build(true, std::time::Duration::from_secs(30))
    }

    fn build(insecure: bool, timeout: std::time::Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(DEFAULT_UA)
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::limited(10))
            .danger_accept_invalid_certs(insecure)
            .build()
            .map_err(|e| Error::Fetch {
                url: "<cliente http>".into(),
                source: Box::new(e),
            })?;
        Ok(Self {
            client,
            session: None,
        })
    }

    /// Reutiliza una sesión fija: en la marcha corta solo se inyectan sus
    /// **cookies** (el localStorage no viaja por HTTP). Additivo.
    pub fn with_session(mut self, session: Session) -> Self {
        self.session = Some(Arc::new(RwLock::new(Some(session))));
        self
    }

    /// Comparte una celda de sesión (la que el re-login refresca en caliente).
    pub fn with_session_cell(mut self, cell: SessionCell) -> Self {
        self.session = Some(cell);
        self
    }
}

#[async_trait]
impl Fetcher for HttpFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        let mut req = self.client.get(url);
        // Marcha corta autenticada: inyecta las cookies de la sesión, si hay.
        // El guard se suelta antes del await (solo se extrae la cadena).
        let cookie = self.session.as_ref().and_then(|cell| {
            cell.read()
                .ok()
                .and_then(|g| g.as_ref().and_then(|s| s.cookie_header()))
        });
        if let Some(cookie) = cookie {
            req = req.header(reqwest::header::COOKIE, cookie);
        }
        let resp = req.send().await.map_err(|e| Error::fetch(url, e))?;
        let status = resp.status().as_u16();
        let final_url = resp.url().to_string();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // El contenido binario (PDF) se devuelve como bytes; el texto, como
        // `html`. La detección de PDF vive en `FetchedPage::is_pdf`.
        let is_pdf = content_type
            .as_deref()
            .map(|c| c.to_ascii_lowercase().contains("application/pdf"))
            .unwrap_or(false)
            || final_url
                .split(['?', '#'])
                .next()
                .unwrap_or("")
                .to_ascii_lowercase()
                .ends_with(".pdf");

        if is_pdf {
            let bytes = resp.bytes().await.map_err(|e| Error::fetch(url, e))?;
            Ok(FetchedPage {
                url: final_url,
                status: Some(status),
                html: String::new(),
                content_type,
                bytes: Some(bytes.to_vec()),
                intercepted: Vec::new(),
            })
        } else {
            let html = resp.text().await.map_err(|e| Error::fetch(url, e))?;
            Ok(FetchedPage {
                url: final_url,
                status: Some(status),
                html,
                content_type,
                bytes: None,
                intercepted: Vec::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    /// Servidor de un golpe que captura la petición y responde HTML.
    fn servidor_captura() -> (String, std::sync::mpsc::Receiver<String>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 8192];
                let n = stream.read(&mut buf).unwrap_or(0);
                let _ = tx.send(String::from_utf8_lossy(&buf[..n]).into_owned());
                let body = "<html><body>ok</body></html>";
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        (format!("http://{addr}/"), rx)
    }

    #[tokio::test]
    async fn inyecta_la_cookie_de_la_sesion() {
        let (url, rx) = servidor_captura();
        let sesion = Session::from_storage_state(
            serde_json::json!({ "cookies": [{ "name": "sid", "value": "abc" }] }),
        );
        let f = HttpFetcher::new().unwrap().with_session(sesion);
        f.fetch(&url).await.unwrap();
        let req = rx.recv().unwrap().to_ascii_lowercase();
        assert!(req.contains("cookie: sid=abc"), "req fue: {req}");
    }

    #[tokio::test]
    async fn sin_sesion_no_manda_cookie() {
        let (url, rx) = servidor_captura();
        let f = HttpFetcher::new().unwrap();
        f.fetch(&url).await.unwrap();
        let req = rx.recv().unwrap().to_ascii_lowercase();
        assert!(!req.contains("cookie:"), "no debería mandar cookie: {req}");
    }
}
