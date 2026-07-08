//! Rutas y handlers de la API.

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use std::sync::Arc;

use crawl4rs_core::{CrawlConfig, CssSelectorStrategy, FieldSpec, SemanticDensityStrategy};

use crate::auth::require_jwt;
use crate::dto::{CrawlAccepted, CrawlRequest, JobResult, PageOut, TokenResponse};
use crate::jobs::{Job, StreamEvent};
use crate::state::AppState;

/// Construye el router de la aplicación.
pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/crawl", post(create_crawl))
        .route("/crawl/{id}/status", get(get_status))
        .route("/crawl/{id}/result", get(get_result))
        .layer(middleware::from_fn_with_state(state.clone(), require_jwt));

    Router::new()
        .route("/health", get(health))
        .route("/dashboard", get(dashboard))
        .route("/auth/token", post(issue_token))
        .route("/crawl/{id}/stream", get(ws_stream))
        .merge(protected)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn dashboard() -> Html<&'static str> {
    Html(crate::dashboard::DASHBOARD_HTML)
}

async fn issue_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<TokenResponse>, StatusCode> {
    let presented = headers.get("x-api-key").and_then(|v| v.to_str().ok());
    if !state.auth.api_key_ok(presented) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = state
        .auth
        .issue("client")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(TokenResponse {
        token,
        token_type: "Bearer".into(),
    }))
}

async fn create_crawl(
    State(state): State<AppState>,
    Json(req): Json<CrawlRequest>,
) -> Result<(StatusCode, Json<CrawlAccepted>), StatusCode> {
    if req.url.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = uuid::Uuid::new_v4().to_string();
    let job = state.jobs.create(id.clone());

    let config = CrawlConfig {
        query: req.query.clone(),
        max_depth: req.max_depth,
        max_pages: req.max_pages.max(1),
        concurrency: req.concurrency.max(1),
        same_domain: !req.cross_domain,
        ..Default::default()
    };

    // Extracción por trabajo: clona el crawler compartido y le añade la
    // estrategia pedida sin afectar a otros trabajos.
    let mut crawler = state.crawler.clone();
    if !req.extract_css.is_empty() {
        let fields = req
            .extract_css
            .iter()
            .map(|(name, sel)| FieldSpec::text(name.clone(), sel.clone()))
            .collect();
        crawler = crawler.with_extraction(std::sync::Arc::new(CssSelectorStrategy::new(fields)));
    } else if req.extract_semantic {
        crawler = crawler.with_extraction(std::sync::Arc::new(SemanticDensityStrategy::new()));
    }

    let url = req.url.clone();
    let job_task = job.clone();
    tokio::spawn(async move {
        run_job(crawler, url, config, job_task).await;
    });

    Ok((StatusCode::ACCEPTED, Json(CrawlAccepted { id })))
}

/// Ejecuta el crawl de un trabajo, emitiendo progreso.
async fn run_job(crawler: crawl4rs_core::Crawler, url: String, config: CrawlConfig, job: Arc<Job>) {
    job.mark_running();
    job.emit(StreamEvent::Started {
        id: job.id().to_string(),
    });

    let job_cb = job.clone();
    let result = crawler
        .crawl_deep_with(&url, &config, move |p| {
            job_cb.set_completed(p.completed);
            job_cb.emit(StreamEvent::Page {
                url: p.url,
                ok: p.ok,
                completed: p.completed,
            });
        })
        .await;

    match result {
        Ok(report) => {
            let pages = report
                .pages
                .into_iter()
                .map(|p| PageOut {
                    url: p.url,
                    fit_markdown: p.fit_markdown,
                    extracted: p.extracted,
                })
                .collect();
            let errors = report
                .errors
                .into_iter()
                .map(|(u, e)| serde_json::json!({ "url": u, "error": e }))
                .collect();
            job.finish(JobResult {
                id: job.id().to_string(),
                pages,
                errors,
            });
        }
        Err(e) => job.fail(e.to_string()),
    }
}

async fn get_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::dto::JobStatus>, StatusCode> {
    match state.jobs.get(&id) {
        Some(job) => Ok(Json(job.status())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_result(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<JobResult>, StatusCode> {
    let job = state.jobs.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    match job.result() {
        Some(result) => Ok(Json(result)),
        // Aún en ejecución: 202 indica "acepta pero no listo".
        None => Err(StatusCode::ACCEPTED),
    }
}

#[derive(Deserialize)]
struct TokenQuery {
    token: String,
}

async fn ws_stream(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(q): Query<TokenQuery>,
) -> Response {
    if state.auth.verify(&q.token).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let Some(job) = state.jobs.get(&id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    ws.on_upgrade(move |socket| stream_job(socket, job))
}

async fn stream_job(mut socket: WebSocket, job: Arc<Job>) {
    let mut rx = job.subscribe();

    // Si el trabajo ya terminó antes de suscribirnos, cerramos con su estado.
    let status = job.status();
    if matches!(
        status.state,
        crate::dto::JobState::Done | crate::dto::JobState::Failed
    ) {
        let ev = match status.error {
            Some(error) => StreamEvent::Failed { error },
            None => StreamEvent::Done {
                completed: status.completed,
            },
        };
        let _ = socket.send(Message::Text(to_text(&ev))).await;
        return;
    }

    while let Ok(ev) = rx.recv().await {
        let terminal = matches!(ev, StreamEvent::Done { .. } | StreamEvent::Failed { .. });
        if socket.send(Message::Text(to_text(&ev))).await.is_err() {
            break;
        }
        if terminal {
            break;
        }
    }
}

fn to_text(ev: &StreamEvent) -> axum::extract::ws::Utf8Bytes {
    serde_json::to_string(ev).unwrap_or_default().into()
}
