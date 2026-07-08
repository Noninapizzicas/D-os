//! # crawl4rs-extract
//!
//! Estrategias para extraer datos estructurados (JSON) desde HTML o Markdown.
//!
//! Todas implementan el trait [`ExtractionStrategy`]. Hoy están disponibles:
//!
//! - [`CssSelectorStrategy`]: extracción por selectores CSS.
//! - [`SemanticDensityStrategy`]: aísla el bloque principal por densidad de
//!   texto (mejora sobre Readability).
//!
//! La estrategia con LLM local (`candle`) llega en la Fase 3.

use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

mod css;
mod jsonld;
mod semantic;

pub use css::{CssSelectorStrategy, FieldSpec};
pub use jsonld::extract_jsonld;
pub use semantic::SemanticDensityStrategy;

/// Errores de extracción.
#[derive(Debug, Error)]
pub enum ExtractError {
    /// Un selector CSS no es válido.
    #[error("selector CSS inválido: {0}")]
    BadSelector(String),

    /// Funcionalidad aún no implementada.
    #[error("no implementado todavía: {0}")]
    NotImplemented(&'static str),
}

/// Resultado de extracción.
pub type Result<T> = std::result::Result<T, ExtractError>;

/// Estrategia de extracción de datos estructurados.
#[async_trait]
pub trait ExtractionStrategy: Send + Sync {
    /// Nombre legible de la estrategia.
    fn name(&self) -> &'static str;

    /// Extrae datos a partir del HTML crudo y/o del Markdown ya convertido.
    async fn extract(&self, html: &str, markdown: &str) -> Result<Value>;
}
