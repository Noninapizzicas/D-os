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
}

impl AppState {
    /// Crea el estado con el crawler y la configuración de auth dados.
    pub fn new(crawler: Crawler, auth: AuthConfig) -> Self {
        Self {
            crawler,
            jobs: Arc::new(JobManager::new()),
            auth,
        }
    }
}
