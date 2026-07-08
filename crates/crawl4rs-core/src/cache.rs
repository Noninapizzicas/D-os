//! Caché de resultados de crawl (feature `cache`).
//!
//! Envuelve [`crawl4rs_cache::TieredCache`] (RAM + `sled` en disco) para
//! guardar [`CrawlResult`] por URL, evitando volver a descargar y procesar
//! páginas ya vistas.

use std::sync::Arc;

use tracing::warn;

use crawl4rs_cache::{DiskCache, TieredCache};

use crate::error::{Error, Result};
use crate::result::CrawlResult;

impl From<crawl4rs_cache::CacheError> for Error {
    fn from(e: crawl4rs_cache::CacheError) -> Self {
        Error::Cache(e.to_string())
    }
}

/// Caché de resultados por URL, compartible entre clones del crawler.
#[derive(Clone)]
pub struct ResultCache {
    inner: Arc<TieredCache<CrawlResult>>,
}

impl ResultCache {
    /// Abre la caché en `dir`, con `memory_capacity` entradas en RAM.
    pub fn open(dir: impl AsRef<std::path::Path>, memory_capacity: usize) -> Result<Self> {
        let disk = DiskCache::open(dir)?;
        Ok(Self {
            inner: Arc::new(TieredCache::new(memory_capacity, disk)),
        })
    }

    /// Recupera el resultado cacheado para una URL, si existe.
    pub fn get(&self, url: &str) -> Option<CrawlResult> {
        match self.inner.get(url) {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e, "fallo leyendo la caché; se ignora");
                None
            }
        }
    }

    /// Almacena el resultado de una URL.
    pub fn put(&self, url: &str, result: &CrawlResult) {
        if let Err(e) = self.inner.put(url, result.clone()) {
            warn!(error = %e, "fallo escribiendo en la caché; se ignora");
        }
    }

    /// Fuerza el volcado a disco.
    pub fn flush(&self) {
        if let Err(e) = self.inner.flush() {
            warn!(error = %e, "fallo al volcar la caché a disco");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CrawlConfig;
    use crate::crawler::Crawler;
    use crate::fetch::{FetchedPage, Fetcher};
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Fetcher que cuenta cuántas veces se le pide descargar.
    struct CountingFetcher {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl Fetcher for CountingFetcher {
        async fn fetch(&self, url: &str) -> Result<FetchedPage> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(FetchedPage {
                url: url.to_string(),
                status: Some(200),
                html: "<article><h1>Cacheable</h1><p>Contenido de prueba con \
                       suficientes palabras para el pipeline.</p></article>"
                    .to_string(),
            })
        }
    }

    #[tokio::test]
    async fn segundo_crawl_no_vuelve_a_descargar() {
        let calls = Arc::new(AtomicUsize::new(0));
        let fetcher = Arc::new(CountingFetcher {
            calls: calls.clone(),
        });
        let dir = tempfile::tempdir().unwrap();
        let cache = ResultCache::open(dir.path(), 16).unwrap();
        let crawler = Crawler::new(fetcher).with_cache(cache);
        let config = CrawlConfig::default();

        let a = crawler.crawl("https://x.test/p", &config).await.unwrap();
        let b = crawler.crawl("https://x.test/p", &config).await.unwrap();

        assert_eq!(a.markdown, b.markdown);
        // El fetcher sólo se invocó una vez: el segundo crawl fue un acierto.
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
