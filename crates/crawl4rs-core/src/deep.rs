//! Crawl profundo: recorre un sitio siguiendo enlaces (BFS o DFS).
//!
//! Reutiliza [`Crawler::crawl`] por página y los `links` que el pipeline de
//! Markdown ya extrae. Respeta `max_pages`, `max_depth`, `deep_strategy` y
//! `same_domain` de [`CrawlConfig`]. El recorrido es secuencial y
//! determinista; la concurrencia masiva llega en la Fase 4.

use std::collections::{HashSet, VecDeque};

use tracing::{debug, instrument, warn};
use url::Url;

use crate::config::{CrawlConfig, DeepStrategy};
use crate::crawler::Crawler;
use crate::error::{Error, Result};
use crate::result::CrawlResult;

/// Resultado de un crawl profundo.
#[derive(Debug, Default)]
pub struct DeepReport {
    /// Páginas procesadas con éxito, en orden de visita.
    pub pages: Vec<CrawlResult>,
    /// Errores por URL (URL, mensaje) que no detuvieron el recorrido.
    pub errors: Vec<(String, String)>,
}

impl DeepReport {
    /// Número de páginas procesadas con éxito.
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// Indica si no se procesó ninguna página.
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}

/// Normaliza una URL para deduplicar: descarta el fragmento (`#...`).
fn normalize(mut url: Url) -> String {
    url.set_fragment(None);
    url.into()
}

impl Crawler {
    /// Recorre en profundidad a partir de `seed`, siguiendo enlaces.
    #[instrument(skip(self, config), fields(seed = %seed))]
    pub async fn crawl_deep(&self, seed: &str, config: &CrawlConfig) -> Result<DeepReport> {
        let seed_url = Url::parse(seed).map_err(|_| Error::InvalidUrl(seed.to_string()))?;
        let seed_host = seed_url.host_str().map(str::to_owned);

        let mut visited: HashSet<String> = HashSet::new();
        let mut frontier: VecDeque<(String, usize)> = VecDeque::new();
        frontier.push_back((normalize(seed_url.clone()), 0));

        let mut report = DeepReport::default();
        let concurrency = config.concurrency.max(1);

        while report.pages.len() < config.max_pages && !frontier.is_empty() {
            // Toma una oleada de URLs nuevas, sin exceder ni la concurrencia
            // ni el presupuesto restante de páginas.
            let cap = concurrency.min(config.max_pages - report.pages.len());
            let mut batch: Vec<(String, usize)> = Vec::with_capacity(cap);
            while batch.len() < cap {
                // BFS toma del frente; DFS del final (LIFO).
                let next = match config.deep_strategy {
                    DeepStrategy::Bfs => frontier.pop_front(),
                    DeepStrategy::Dfs => frontier.pop_back(),
                };
                let Some((url, depth)) = next else { break };
                if visited.insert(url.clone()) {
                    batch.push((url, depth));
                }
            }
            if batch.is_empty() {
                break;
            }

            // Descarga la oleada en paralelo; `join_all` preserva el orden.
            let fetched = futures::future::join_all(
                batch
                    .iter()
                    .map(|(url, depth)| async move { (*depth, self.crawl(url, config).await) }),
            )
            .await;

            for ((url, depth), (_, res)) in batch.into_iter().zip(fetched) {
                match res {
                    Ok(result) => {
                        if depth < config.max_depth {
                            self.enqueue_links(
                                &result,
                                depth,
                                &seed_host,
                                config,
                                &visited,
                                &mut frontier,
                            );
                        }
                        report.pages.push(result);
                    }
                    Err(e) => {
                        warn!(url = %url, error = %e, "página omitida en crawl profundo");
                        report.errors.push((url, e.to_string()));
                    }
                }
            }
        }

        debug!(
            paginas = report.pages.len(),
            errores = report.errors.len(),
            "crawl profundo terminado"
        );
        Ok(report)
    }

    /// Resuelve y encola los enlaces de una página según la configuración.
    fn enqueue_links(
        &self,
        result: &CrawlResult,
        depth: usize,
        seed_host: &Option<String>,
        config: &CrawlConfig,
        visited: &HashSet<String>,
        frontier: &mut VecDeque<(String, usize)>,
    ) {
        let base = match Url::parse(&result.url) {
            Ok(u) => u,
            Err(_) => return,
        };

        for link in &result.links {
            let Ok(abs) = base.join(link) else { continue };
            // Sólo seguimos navegación web.
            if !matches!(abs.scheme(), "http" | "https") {
                continue;
            }
            if config.same_domain && abs.host_str().map(str::to_owned) != *seed_host {
                continue;
            }
            let normalized = normalize(abs);
            if visited.contains(&normalized) {
                continue;
            }
            frontier.push_back((normalized, depth + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::{FetchedPage, Fetcher};
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Fetcher de prueba que sirve HTML distinto según la URL.
    struct MapFetcher {
        pages: HashMap<String, String>,
    }

    #[async_trait]
    impl Fetcher for MapFetcher {
        async fn fetch(&self, url: &str) -> Result<FetchedPage> {
            match self.pages.get(url) {
                Some(html) => Ok(FetchedPage {
                    url: url.to_string(),
                    status: Some(200),
                    html: html.clone(),
                }),
                None => Err(Error::fetch(
                    url,
                    std::io::Error::new(std::io::ErrorKind::NotFound, "404"),
                )),
            }
        }
    }

    fn site() -> Arc<MapFetcher> {
        let mut pages = HashMap::new();
        pages.insert(
            "https://sitio.test/".to_string(),
            r#"<h1>Inicio</h1>
               <a href="/a">A</a> <a href="/b">B</a>
               <a href="https://externo.test/x">externo</a>"#
                .to_string(),
        );
        pages.insert(
            "https://sitio.test/a".to_string(),
            r#"<h1>Página A</h1><a href="/c">C</a> <a href="/">Inicio</a>"#.to_string(),
        );
        pages.insert(
            "https://sitio.test/b".to_string(),
            r#"<h1>Página B</h1>"#.to_string(),
        );
        pages.insert(
            "https://sitio.test/c".to_string(),
            r#"<h1>Página C</h1>"#.to_string(),
        );
        Arc::new(MapFetcher { pages })
    }

    #[tokio::test]
    async fn bfs_respeta_dominio_y_profundidad() {
        let crawler = Crawler::new(site());
        let config = CrawlConfig {
            max_depth: 2,
            word_count_threshold: 1,
            ..Default::default()
        };
        let report = crawler
            .crawl_deep("https://sitio.test/", &config)
            .await
            .unwrap();

        let urls: HashSet<_> = report.pages.iter().map(|p| p.url.as_str()).collect();
        // Debe alcanzar inicio, /a, /b (profundidad 1) y /c (profundidad 2).
        assert!(urls.contains("https://sitio.test/"));
        assert!(urls.contains("https://sitio.test/a"));
        assert!(urls.contains("https://sitio.test/b"));
        assert!(urls.contains("https://sitio.test/c"));
        // El enlace externo no se sigue (same_domain = true por defecto).
        assert!(!urls.iter().any(|u| u.contains("externo.test")));
        // Sin duplicados pese al enlace de vuelta a inicio desde /a.
        assert_eq!(urls.len(), report.pages.len());
    }

    #[tokio::test]
    async fn max_pages_limita_el_recorrido() {
        let crawler = Crawler::new(site());
        let config = CrawlConfig {
            max_pages: 2,
            max_depth: 5,
            word_count_threshold: 1,
            ..Default::default()
        };
        let report = crawler
            .crawl_deep("https://sitio.test/", &config)
            .await
            .unwrap();
        assert_eq!(report.pages.len(), 2);
    }

    #[tokio::test]
    async fn profundidad_cero_solo_visita_la_semilla() {
        let crawler = Crawler::new(site());
        let config = CrawlConfig {
            max_depth: 0,
            word_count_threshold: 1,
            ..Default::default()
        };
        let report = crawler
            .crawl_deep("https://sitio.test/", &config)
            .await
            .unwrap();
        assert_eq!(report.pages.len(), 1);
        assert_eq!(report.pages[0].url, "https://sitio.test/");
    }
}
