//! Abstracción de descarga de páginas.
//!
//! El núcleo no impone _cómo_ se obtiene el HTML: puede ser vía navegador
//! headless (`chromiumoxide`, Fase 1), un cliente HTTP simple, o datos
//! estáticos en tests. Todo lo que necesita el orquestador es un
//! [`Fetcher`].

use async_trait::async_trait;

#[cfg(not(feature = "browser"))]
use crate::error::Error;
use crate::error::Result;

/// Una página descargada, sin procesar.
#[derive(Debug, Clone, Default)]
pub struct FetchedPage {
    /// URL efectiva tras redirecciones.
    pub url: String,
    /// Código de estado HTTP, si aplica.
    pub status: Option<u16>,
    /// Cuerpo como texto (HTML). Vacío para contenido binario (p. ej. PDF).
    pub html: String,
    /// `Content-Type` de la respuesta, si se conoce.
    pub content_type: Option<String>,
    /// Bytes crudos cuando el contenido no es HTML (p. ej. PDF).
    pub bytes: Option<Vec<u8>>,
}

impl FetchedPage {
    /// Indica si la respuesta parece un PDF (por `Content-Type` o extensión).
    pub fn is_pdf(&self) -> bool {
        let by_ct = self
            .content_type
            .as_deref()
            .map(|c| c.to_ascii_lowercase().contains("application/pdf"))
            .unwrap_or(false);
        by_ct
            || self
                .url
                .split(['?', '#'])
                .next()
                .map(|p| p.to_ascii_lowercase().ends_with(".pdf"))
                .unwrap_or(false)
    }
}

/// Fuente de HTML para el orquestador.
#[async_trait]
pub trait Fetcher: Send + Sync {
    /// Descarga la URL indicada.
    async fn fetch(&self, url: &str) -> Result<FetchedPage>;
}

/// Fetcher que sirve HTML fijo, independientemente de la URL.
///
/// Útil para tests y para procesar HTML ya obtenido por otros medios.
#[derive(Debug, Clone)]
pub struct StaticFetcher {
    html: String,
    status: Option<u16>,
}

impl StaticFetcher {
    /// Crea un fetcher que siempre devuelve `html`.
    pub fn new(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            status: Some(200),
        }
    }
}

#[async_trait]
impl Fetcher for StaticFetcher {
    async fn fetch(&self, url: &str) -> Result<FetchedPage> {
        Ok(FetchedPage {
            url: url.to_string(),
            status: self.status,
            html: self.html.clone(),
            ..Default::default()
        })
    }
}

/// Fetcher basado en navegador headless (stub sin la feature `browser`).
///
/// La implementación real vive en [`crate::browser::BrowserFetcher`]; este
/// marcador sólo existe en builds compilados con `--no-default-features`.
#[cfg(not(feature = "browser"))]
#[derive(Debug, Default, Clone)]
pub struct BrowserFetcher {
    _private: (),
}

#[cfg(not(feature = "browser"))]
impl BrowserFetcher {
    /// Crea el fetcher (sin backend en este build).
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(not(feature = "browser"))]
#[async_trait]
impl Fetcher for BrowserFetcher {
    async fn fetch(&self, _url: &str) -> Result<FetchedPage> {
        Err(Error::NotImplemented(
            "compilado sin la feature `browser`; actívala para usar Chromium",
        ))
    }
}
