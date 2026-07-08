//! # crawl4rs-core
//!
//! Núcleo de Crawl4RS. Orquesta el flujo completo de un crawl:
//! descarga (`Fetcher`) → limpieza → Markdown → `fit_markdown`.
//!
//! ```
//! use std::sync::Arc;
//! use crawl4rs_core::{Crawler, CrawlConfig, StaticFetcher};
//!
//! # async fn demo() -> crawl4rs_core::Result<()> {
//! let html = "<html><body><article><h1>Hola</h1><p>Mundo</p></article></body></html>";
//! let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
//! let result = crawler.crawl("https://ejemplo.test", &CrawlConfig::default()).await?;
//! assert!(result.markdown.contains("Hola"));
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod crawler;
pub mod error;
pub mod fetch;
pub mod result;

pub use config::{CrawlConfig, DeepStrategy};
pub use crawler::Crawler;
pub use error::{Error, Result};
pub use fetch::{BrowserFetcher, FetchedPage, Fetcher, StaticFetcher};
pub use result::CrawlResult;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn crawl_estatico_produce_markdown() {
        let html = "<html><body><article><h1>Título</h1><p>Un párrafo con \
                    suficientes palabras para superar el umbral por defecto.</p>\
                    </article></body></html>";
        let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
        let result = crawler
            .crawl("https://ejemplo.test/a", &CrawlConfig::default())
            .await
            .expect("el crawl debe tener éxito");

        assert!(result.is_success());
        assert!(result.markdown.contains("Título"));
        assert!(result.markdown.contains("párrafo"));
    }

    #[tokio::test]
    async fn browser_fetcher_no_implementado() {
        let crawler = Crawler::new(Arc::new(BrowserFetcher::new()));
        let err = crawler
            .crawl("https://ejemplo.test", &CrawlConfig::default())
            .await
            .expect_err("el navegador aún no está implementado");
        assert!(matches!(err, Error::NotImplemented(_)));
    }
}
