//! Configuración de un crawl.

use serde::{Deserialize, Serialize};

/// Estrategia de recorrido para un crawl profundo (`crawl4rs deep`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeepStrategy {
    /// Búsqueda en anchura.
    #[default]
    Bfs,
    /// Búsqueda en profundidad.
    Dfs,
}

/// Parámetros que controlan un crawl individual.
///
/// Se construye con valores por defecto sensatos y se ajusta con el patrón
/// _builder_:
///
/// ```
/// use crawl4rs_core::CrawlConfig;
///
/// let config = CrawlConfig::default()
///     .with_query(Some("rust async runtime".to_string()))
///     .with_word_count_threshold(20);
/// assert_eq!(config.word_count_threshold, 20);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CrawlConfig {
    /// Consulta usada por los filtros de relevancia (p. ej. BM25). `None`
    /// desactiva el filtrado por consulta y se produce sólo `fit_markdown`
    /// basado en poda heurística.
    pub query: Option<String>,

    /// Umbral mínimo de palabras para que un bloque de texto se conserve
    /// durante la limpieza/poda.
    pub word_count_threshold: usize,

    /// Si se deben excluir enlaces externos del Markdown resultante.
    pub exclude_external_links: bool,

    /// Número máximo de páginas a visitar en un crawl profundo.
    pub max_pages: usize,

    /// Profundidad máxima en un crawl profundo (0 = sólo la URL semilla).
    pub max_depth: usize,

    /// Estrategia de recorrido para crawl profundo.
    pub deep_strategy: DeepStrategy,

    /// Tiempo máximo de espera por página, en milisegundos.
    pub timeout_ms: u64,

    /// Activa el modo stealth (anti-detección). Ver `crawl4rs-stealth`.
    pub stealth: bool,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            query: None,
            word_count_threshold: 10,
            exclude_external_links: false,
            max_pages: 100,
            max_depth: 2,
            deep_strategy: DeepStrategy::default(),
            timeout_ms: 30_000,
            stealth: false,
        }
    }
}

impl CrawlConfig {
    /// Fija la consulta para los filtros de relevancia.
    pub fn with_query(mut self, query: Option<String>) -> Self {
        self.query = query;
        self
    }

    /// Fija el umbral mínimo de palabras por bloque.
    pub fn with_word_count_threshold(mut self, threshold: usize) -> Self {
        self.word_count_threshold = threshold;
        self
    }

    /// Activa o desactiva el modo stealth.
    pub fn with_stealth(mut self, stealth: bool) -> Self {
        self.stealth = stealth;
        self
    }
}
