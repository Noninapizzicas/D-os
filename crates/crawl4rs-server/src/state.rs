//! Estado compartido de la aplicación.

use std::sync::Arc;

use crawl4rs_core::Crawler;

use crate::auth::AuthConfig;
use crate::jobs::JobManager;

/// Estado inyectado en todos los handlers de axum.
#[derive(Clone)]
pub struct AppState {
    /// Crawler compartido (reutiliza el mismo fetcher/pool entre trabajos).
    pub crawler: Crawler,
    /// Registro de trabajos en memoria.
    pub jobs: Arc<JobManager>,
    /// Configuración de autenticación.
    pub auth: AuthConfig,
    /// URL base de SearXNG para `POST /search`. `None` → el endpoint degrada
    /// con 503.
    pub searxng_url: Option<String>,
}

impl AppState {
    /// Crea el estado con el crawler y la configuración de auth dados.
    pub fn new(crawler: Crawler, auth: AuthConfig) -> Self {
        Self {
            crawler,
            jobs: Arc::new(JobManager::new()),
            auth,
            searxng_url: None,
        }
    }

    /// Configura la URL de SearXNG (habilita `POST /search`).
    pub fn with_searxng(mut self, url: Option<String>) -> Self {
        self.searxng_url = url.filter(|u| !u.trim().is_empty());
        self
    }
}
