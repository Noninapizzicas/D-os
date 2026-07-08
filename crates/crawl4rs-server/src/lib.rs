//! # crawl4rs-server
//!
//! Contrato de la API HTTP/WebSocket de Crawl4RS. Los tipos de
//! petición/respuesta ya están definidos para poder programar contra ellos;
//! la implementación con `axum` + `tower` (rutas, WebSockets, JWT, dashboard)
//! llega en la Fase 6 de la hoja de ruta.
//!
//! Endpoints previstos:
//! - `POST /crawl` — inicia un crawl.
//! - `GET /crawl/{id}/status` — estado en tiempo real.
//! - `WS /crawl/{id}/stream` — streaming de logs y capturas.
//! - `GET /dashboard` — interfaz web de monitoreo.

use serde::{Deserialize, Serialize};

/// Cuerpo de `POST /crawl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlRequest {
    /// URLs a procesar.
    pub urls: Vec<String>,
    /// Consulta opcional para el filtrado por relevancia.
    #[serde(default)]
    pub query: Option<String>,
    /// Activar modo stealth.
    #[serde(default)]
    pub stealth: bool,
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
    /// Total de páginas.
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializa_estado_en_minusculas() {
        let s = serde_json::to_string(&JobState::Running).unwrap();
        assert_eq!(s, "\"running\"");
    }
}
