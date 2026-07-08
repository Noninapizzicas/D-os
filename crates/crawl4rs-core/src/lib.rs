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

#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "cache")]
pub mod cache;
pub mod config;
pub mod crawler;
pub mod deep;
pub mod error;
pub mod fetch;
pub mod result;

#[cfg(feature = "browser")]
pub use browser::{
    detect_chrome_executable, BrowserFetcher, BrowserPool, BrowserPoolConfig, Fingerprint,
    SessionManager, StealthConfig, StealthEngine,
};
#[cfg(feature = "cache")]
pub use cache::ResultCache;
pub use config::{CrawlConfig, DeepStrategy};
#[cfg(feature = "extract")]
pub use crawl4rs_extract::{
    CssSelectorStrategy, ExtractionStrategy, FieldSpec, SemanticDensityStrategy,
};
pub use crawler::Crawler;
pub use deep::{DeepProgress, DeepReport};
pub use error::{Error, Result};
#[cfg(not(feature = "browser"))]
pub use fetch::BrowserFetcher;
pub use fetch::{FetchedPage, Fetcher, StaticFetcher};
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

    #[cfg(feature = "extract")]
    #[tokio::test]
    async fn extraccion_css_puebla_extracted() {
        use crate::{CssSelectorStrategy, FieldSpec};
        let html = r#"<html><body><article><h1 class="t">Producto X</h1>
            <span class="price">9.99</span>
            <p>Descripción con palabras suficientes para el pipeline.</p>
            </article></body></html>"#;
        let strat = CssSelectorStrategy::new(vec![
            FieldSpec::text("titulo", ".t"),
            FieldSpec::text("precio", ".price"),
        ]);
        let crawler =
            Crawler::new(Arc::new(StaticFetcher::new(html))).with_extraction(Arc::new(strat));
        let result = crawler
            .crawl("https://tienda.test/x", &CrawlConfig::default())
            .await
            .unwrap();

        let extracted = result.extracted.expect("debe haber datos extraídos");
        assert_eq!(extracted["titulo"], "Producto X");
        assert_eq!(extracted["precio"], "9.99");
    }

    #[cfg(not(feature = "browser"))]
    #[tokio::test]
    async fn browser_fetcher_stub_sin_feature() {
        let crawler = Crawler::new(Arc::new(BrowserFetcher::new()));
        let err = crawler
            .crawl("https://ejemplo.test", &CrawlConfig::default())
            .await
            .expect_err("sin la feature `browser` no hay backend");
        assert!(matches!(err, Error::NotImplemented(_)));
    }
}
