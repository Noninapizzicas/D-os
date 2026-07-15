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

use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::fetch::{FetchedPage, Fetcher};

/// Fetcher que abre la página a través del wrapper HTTP de Playwright.
#[derive(Clone)]
pub struct PlaywrightFetcher {
    client: reqwest::Client,
    endpoint: String,
}

/// Cuerpo de `POST /abrir`.
#[derive(Serialize)]
struct AbrirReq<'a> {
    url: &'a str,
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
        Ok(Self { client, endpoint })
    }
}

#[async_trait]
impl Fetcher for PlaywrightFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        let body = serde_json::to_string(&AbrirReq { url })
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    /// Servidor HTTP mínimo de un solo golpe que devuelve `body` fijo. Evita
    /// depender de un mock externo para probar la costura.
    fn servidor_una_vez(body: &'static str) -> String {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf);
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        format!("http://{addr}")
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
}
