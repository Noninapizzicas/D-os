//! Tipos de error del núcleo.

use thiserror::Error;

/// Alias de resultado usado en toda la crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errores que puede producir el orquestador de crawl.
#[derive(Debug, Error)]
pub enum Error {
    /// La URL no es válida.
    #[error("URL inválida: {0}")]
    InvalidUrl(String),

    /// Falló la descarga de la página (red, DNS, timeout, código HTTP).
    #[error("fallo al descargar {url}: {source}")]
    Fetch {
        /// URL que se intentaba descargar.
        url: String,
        /// Causa subyacente.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Falló el pipeline de conversión a Markdown.
    #[error("fallo en el pipeline de markdown: {0}")]
    Markdown(String),

    /// Fallo del navegador headless (lanzamiento, CDP, timeout).
    #[error("fallo del navegador: {0}")]
    Browser(String),

    /// Fallo de la capa de caché (disco/serialización).
    #[error("fallo de caché: {0}")]
    Cache(String),

    /// Funcionalidad planificada pero aún no implementada (ver hoja de ruta).
    #[error("no implementado todavía: {0}")]
    NotImplemented(&'static str),
}

impl Error {
    /// Construye un [`Error::Fetch`] a partir de cualquier error compatible.
    pub fn fetch(
        url: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Error::Fetch {
            url: url.into(),
            source: Box::new(source),
        }
    }
}
