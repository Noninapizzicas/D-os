//! Fetcher HTTP rápido, sin navegador (feature `http`).
//!
//! Para páginas simples (la mayoría), abrir Chromium es un derroche de RAM y
//! latencia en un VPS pequeño. Este fetcher usa `reqwest`: una petición GET
//! con cabeceras realistas, redirecciones seguidas y descompresión. El
//! orquestador lo usa como camino por defecto y sólo escala a navegador
//! cuando hace falta (ver [`crate::config::FetchMode`]).

use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::fetch::{FetchedPage, Fetcher};

const DEFAULT_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

/// Fetcher HTTP basado en `reqwest`.
#[derive(Clone)]
pub struct HttpFetcher {
    client: reqwest::Client,
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
        Ok(Self { client })
    }
}

#[async_trait]
impl Fetcher for HttpFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::fetch(url, e))?;
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
            })
        } else {
            let html = resp.text().await.map_err(|e| Error::fetch(url, e))?;
            Ok(FetchedPage {
                url: final_url,
                status: Some(status),
                html,
                content_type,
                bytes: None,
            })
        }
    }
}
