//! Búsqueda web como front-end a SearXNG.
//!
//! Si no hay `SEARXNG_URL` configurado, el endpoint degrada con 503 (motivo
//! explícito); no inventa resultados.

use serde_json::Value;

use crate::dto::SearchResult;

/// Errores de búsqueda.
#[derive(Debug)]
pub enum SearchError {
    /// SearXNG no está configurado (degradación honesta → 503).
    NotConfigured,
    /// Fallo al contactar o parsear SearXNG.
    Upstream(String),
}

/// Consulta SearXNG (`/search?format=json`) y normaliza los resultados.
pub async fn search(
    searxng_url: Option<&str>,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, SearchError> {
    let base = searxng_url.ok_or(SearchError::NotConfigured)?;
    let endpoint = format!("{}/search", base.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let resp = client
        .get(&endpoint)
        .query(&[("q", query), ("format", "json")])
        .send()
        .await
        .map_err(|e| SearchError::Upstream(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(SearchError::Upstream(format!(
            "SearXNG devolvió {}",
            resp.status()
        )));
    }
    let body: Value = resp
        .json()
        .await
        .map_err(|e| SearchError::Upstream(e.to_string()))?;

    let results = body
        .get("results")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .take(limit)
                .map(|r| SearchResult {
                    title: str_field(r, "title"),
                    url: str_field(r, "url"),
                    snippet: str_field(r, "content"),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(results)
}

fn str_field(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}
