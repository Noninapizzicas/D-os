//! Abstracción de descarga de páginas.
//!
//! El núcleo no impone _cómo_ se obtiene el HTML: puede ser vía navegador
//! headless (`chromiumoxide`, Fase 1), un cliente HTTP simple, o datos
//! estáticos en tests. Todo lo que necesita el orquestador es un
//! [`Fetcher`].

use async_trait::async_trait;

use crate::error::{Error, Result};

/// Una página descargada, sin procesar.
#[derive(Debug, Clone)]
pub struct FetchedPage {
    /// URL efectiva tras redirecciones.
    pub url: String,
    /// Código de estado HTTP, si aplica.
    pub status: Option<u16>,
    /// Cuerpo HTML.
    pub html: String,
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
        })
    }
}

/// Fetcher basado en navegador headless (Chromium vía CDP).
///
/// Marcador de posición para la Fase 1 de la hoja de ruta. Hoy devuelve
/// [`Error::NotImplemented`].
#[derive(Debug, Default, Clone)]
pub struct BrowserFetcher {
    _private: (),
}

impl BrowserFetcher {
    /// Crea el fetcher (aún sin backend real).
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Fetcher for BrowserFetcher {
    async fn fetch(&self, _url: &str) -> Result<FetchedPage> {
        Err(Error::NotImplemented(
            "BrowserFetcher (chromiumoxide) llega en la Fase 1",
        ))
    }
}
