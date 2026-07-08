//! Tipos de petición/respuesta de la API.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crawl4rs_core::FetchMode;

/// Especificación de un campo de extracción por CSS. Admite dos formas
/// (aditivo, sin romper clientes existentes):
/// - `"h1"` → texto del selector.
/// - `{ "selector": "img", "attr": "src", "many": true }` → atributo/colección.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum FieldSpecDto {
    /// Selector simple: extrae el texto de la primera coincidencia.
    Text(String),
    /// Selector con atributo y/o colección.
    Detailed {
        /// Selector CSS.
        selector: String,
        /// Atributo a leer (`src`, `href`, `data-...`); `None` → texto.
        #[serde(default)]
        attr: Option<String>,
        /// Si `true`, recoge todas las coincidencias en un array.
        #[serde(default)]
        many: bool,
    },
}

/// Cuerpo de `POST /crawl`.
#[derive(Debug, Clone, Deserialize)]
pub struct CrawlRequest {
    /// URL semilla a procesar.
    pub url: String,
    /// Consulta opcional para el filtrado por relevancia (BM25).
    #[serde(default)]
    pub query: Option<String>,
    /// Modo de descarga: `fast` | `browser` | `auto` (por defecto).
    #[serde(default)]
    pub mode: FetchMode,
    /// Profundidad máxima (0 = sólo la semilla).
    #[serde(default)]
    pub max_depth: usize,
    /// Máximo de páginas a visitar.
    #[serde(default = "default_max_pages")]
    pub max_pages: usize,
    /// Descargas simultáneas.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    /// Seguir enlaces a otros dominios.
    #[serde(default)]
    pub cross_domain: bool,
    /// Extracción por CSS: mapa `nombre → selector | {selector, attr, many}`.
    /// Si está presente, cada página incluye los campos en `extracted`.
    #[serde(default)]
    pub extract_css: std::collections::HashMap<String, FieldSpecDto>,
    /// Extrae el contenido principal por densidad semántica.
    #[serde(default)]
    pub extract_semantic: bool,
    /// Extrae los objetos JSON-LD / schema.org de cada página.
    #[serde(default)]
    pub extract_jsonld: bool,
}

fn default_max_pages() -> usize {
    25
}
fn default_concurrency() -> usize {
    4
}

/// Estado de un trabajo de crawl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState {
    /// En cola, aún no iniciado.
    Queued,
    /// En ejecución.
    Running,
    /// Terminado con éxito.
    Done,
    /// Terminado con error.
    Failed,
}

/// Respuesta de `GET /crawl/{id}/status`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    /// Identificador del trabajo.
    pub id: String,
    /// Estado actual.
    pub state: JobState,
    /// Páginas completadas.
    pub completed: usize,
    /// Mensaje de error, si el estado es `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Respuesta de `POST /crawl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlAccepted {
    /// Identificador del trabajo creado.
    pub id: String,
}

/// Respuesta de `POST /auth/token`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// El JWT emitido.
    pub token: String,
    /// Tipo de token (siempre `Bearer`).
    pub token_type: String,
}

/// Una página en el resultado de un trabajo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageOut {
    /// URL efectiva.
    pub url: String,
    /// Markdown filtrado, listo para LLM.
    pub fit_markdown: String,
    /// Datos estructurados extraídos, si se pidió extracción.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted: Option<Value>,
    /// Objetos JSON-LD, si se pidió `extract_jsonld`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jsonld: Vec<Value>,
}

/// Cuerpo de `POST /search`.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchRequest {
    /// Consulta de búsqueda.
    pub query: String,
    /// Máximo de resultados (por defecto 10).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Un resultado de búsqueda.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Título del resultado.
    pub title: String,
    /// URL.
    pub url: String,
    /// Fragmento/resumen.
    pub snippet: String,
}

/// Cuerpo de `POST /map`.
#[derive(Debug, Clone, Deserialize)]
pub struct MapRequest {
    /// URL a mapear.
    pub url: String,
    /// Modo de descarga (por defecto `auto`).
    #[serde(default)]
    pub mode: FetchMode,
}

/// Respuesta de `POST /map`.
#[derive(Debug, Clone, Serialize)]
pub struct MapResponse {
    /// URL efectiva mapeada.
    pub url: String,
    /// Enlaces encontrados (absolutos, únicos).
    pub links: Vec<String>,
}

/// Respuesta de `GET /crawl/{id}/result`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    /// Identificador del trabajo.
    pub id: String,
    /// Páginas procesadas.
    pub pages: Vec<PageOut>,
    /// Errores por URL.
    pub errors: Vec<Value>,
}
