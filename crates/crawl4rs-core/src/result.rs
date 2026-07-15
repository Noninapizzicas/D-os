//! Estructuras de salida de un crawl.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Resultado de descargar y procesar una única página.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    /// URL efectiva de la que proviene el contenido (tras redirecciones).
    pub url: String,

    /// Código de estado HTTP, si está disponible.
    pub status: Option<u16>,

    /// HTML crudo tal como se recibió.
    pub html: String,

    /// Markdown completo, convertido del HTML limpio.
    pub markdown: String,

    /// Markdown "fit": la versión filtrada/podada, lista para un LLM.
    pub fit_markdown: String,

    /// Datos estructurados extraídos, si se aplicó alguna estrategia de
    /// extracción.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted: Option<Value>,

    /// Objetos JSON-LD / schema.org de la página, si se pidió `extract_jsonld`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jsonld: Vec<Value>,

    /// Enlaces encontrados en la página, útiles para crawl profundo.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,

    /// JSON interceptado de la API interna (marcha larga con `interceptar`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intercepted: Vec<Value>,
}

impl CrawlResult {
    /// Indica si la descarga se considera exitosa (2xx o sin estado conocido).
    pub fn is_success(&self) -> bool {
        matches!(self.status, None | Some(200..=299))
    }
}
