//! # crawl4rs-server
//!
//! API HTTP/WebSocket y dashboard de Crawl4RS, construida con `axum`.
//!
//! Endpoints:
//! - `POST /auth/token` — emite un JWT (requiere `x-api-key` si está configurada).
//! - `POST /crawl` — inicia un trabajo de crawl (protegido). Devuelve `{ id }`.
//! - `GET /crawl/{id}/status` — estado del trabajo (protegido).
//! - `GET /crawl/{id}/result` — resultado del trabajo (protegido).
//! - `GET /crawl/{id}/stream` — streaming de progreso por WebSocket
//!   (autenticado con `?token=<jwt>`).
//! - `GET /dashboard` — interfaz web de monitoreo.
//! - `GET /health` — sonda de salud.
//!
//! El servidor es agnóstico al fetcher: recibe un [`crawl4rs_core::Crawler`]
//! ya construido, así que sirve tanto con navegador headless como con HTML
//! estático en tests.

mod auth;
mod dashboard;
mod dto;
mod jobs;
mod routes;
mod state;

use std::net::SocketAddr;

use crawl4rs_core::Crawler;

pub use auth::AuthConfig;
pub use dto::{
    CrawlAccepted, CrawlRequest, JobResult, JobState, JobStatus, PageOut, TokenResponse,
};
pub use routes::router;
pub use state::AppState;

/// Arranca el servidor en `addr` con el crawler y la configuración de auth
/// dados. Bloquea hasta que el servidor termina.
pub async fn serve(addr: SocketAddr, crawler: Crawler, auth: AuthConfig) -> std::io::Result<()> {
    let state = AppState::new(crawler, auth);
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "servidor Crawl4RS escuchando");
    axum::serve(listener, app).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use crawl4rs_core::{Crawler, StaticFetcher};
    use http_body_util::BodyExt;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn test_app() -> axum::Router {
        let html = "<html><body><article><h1>Servido</h1><p>Contenido de prueba \
                    con palabras suficientes para el pipeline.</p></article></body></html>";
        let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
        let state = AppState::new(crawler, AuthConfig::new("secreto-de-test"));
        router(state)
    }

    async fn body_json(resp: axum::response::Response) -> serde_json::Value {
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
    }

    #[tokio::test]
    async fn health_responde_ok() {
        let resp = test_app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn crawl_requiere_token() {
        let resp = test_app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/crawl")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"url":"https://x.test"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn flujo_completo_token_crawl_estado_resultado() {
        let app = test_app();

        // 1) Obtener token (sin api key configurada → emisión abierta).
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let token = body_json(resp).await["token"].as_str().unwrap().to_string();

        // 2) Crear trabajo.
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/crawl")
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"url":"https://x.test/a","max_depth":0}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        let id = body_json(resp).await["id"].as_str().unwrap().to_string();

        // 3) Sondear el estado hasta que termine.
        let mut result = serde_json::Value::Null;
        for _ in 0..50 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/crawl/{id}/result"))
                        .header("authorization", format!("Bearer {token}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            if resp.status() == StatusCode::OK {
                result = body_json(resp).await;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }

        let pages = result["pages"].as_array().expect("resultado con páginas");
        assert_eq!(pages.len(), 1);
        assert!(pages[0]["fit_markdown"]
            .as_str()
            .unwrap()
            .contains("Servido"));
    }

    #[tokio::test]
    async fn crawl_con_extraccion_css() {
        let app = test_app();
        let token = {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/auth/token")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            body_json(resp).await["token"].as_str().unwrap().to_string()
        };

        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/crawl")
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"url":"https://x.test/a","max_depth":0,"extract_css":{"titulo":"h1"}}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let id = body_json(resp).await["id"].as_str().unwrap().to_string();

        let mut result = serde_json::Value::Null;
        for _ in 0..50 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/crawl/{id}/result"))
                        .header("authorization", format!("Bearer {token}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            if resp.status() == StatusCode::OK {
                result = body_json(resp).await;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }

        assert_eq!(result["pages"][0]["extracted"]["titulo"], "Servido");
    }
}
