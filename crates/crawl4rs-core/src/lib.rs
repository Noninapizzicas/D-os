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
#[cfg(feature = "http")]
pub mod http;
pub mod result;

#[cfg(feature = "browser")]
pub use browser::{
    detect_chrome_executable, BrowserFetcher, BrowserPool, BrowserPoolConfig, Fingerprint,
    SessionManager, StealthConfig, StealthEngine,
};
#[cfg(feature = "cache")]
pub use cache::ResultCache;
pub use config::{CrawlConfig, DeepStrategy, FetchMode};
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
#[cfg(feature = "http")]
pub use http::HttpFetcher;
pub use result::CrawlResult;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// Fetcher que devuelve HTML/estado fijos y cuenta sus invocaciones.
    struct FixedFetcher {
        html: String,
        status: u16,
        calls: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl Fetcher for FixedFetcher {
        async fn fetch(&self, url: &str) -> Result<FetchedPage> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(FetchedPage {
                url: url.to_string(),
                status: Some(self.status),
                html: self.html.clone(),
            })
        }
    }

    fn fixed(html: &str, status: u16) -> (Arc<FixedFetcher>, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        (
            Arc::new(FixedFetcher {
                html: html.to_string(),
                status,
                calls: calls.clone(),
            }),
            calls,
        )
    }

    const OK_HTML: &str = "<html><body><article><h1>Contenido</h1><p>Un párrafo \
        con suficientes palabras para superar el umbral por defecto del pipeline.</p>\
        </article></body></html>";

    #[tokio::test]
    async fn auto_escala_a_navegador_ante_challenge() {
        // HTTP devuelve un 403 (challenge); el navegador, contenido real.
        let (http, http_calls) = fixed("<html><body>Just a moment...</body></html>", 403);
        let (browser, browser_calls) = fixed(OK_HTML, 200);
        let crawler = Crawler::new(http.clone())
            .with_http_fetcher(http)
            .with_browser_fetcher(browser);

        let cfg = CrawlConfig {
            mode: FetchMode::Auto,
            ..Default::default()
        };
        let r = crawler.crawl("https://x.test", &cfg).await.unwrap();
        assert!(r.markdown.contains("Contenido"), "usó el navegador");
        assert_eq!(http_calls.load(Ordering::SeqCst), 1);
        assert_eq!(browser_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn auto_usa_http_sin_challenge_y_no_abre_navegador() {
        let (http, http_calls) = fixed(OK_HTML, 200);
        let (browser, browser_calls) = fixed(OK_HTML, 200);
        let crawler = Crawler::new(http.clone())
            .with_http_fetcher(http)
            .with_browser_fetcher(browser);

        let r = crawler
            .crawl(
                "https://x.test",
                &CrawlConfig {
                    mode: FetchMode::Auto,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(r.markdown.contains("Contenido"));
        assert_eq!(http_calls.load(Ordering::SeqCst), 1);
        // El navegador NUNCA se abre: el ahorro de RAM/latencia del linchpin.
        assert_eq!(browser_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn modo_fast_no_escala_aunque_haya_challenge() {
        let (http, http_calls) = fixed("<html><body>Just a moment...</body></html>", 403);
        let (browser, browser_calls) = fixed(OK_HTML, 200);
        let crawler = Crawler::new(http.clone())
            .with_http_fetcher(http)
            .with_browser_fetcher(browser);

        let _ = crawler
            .crawl(
                "https://x.test",
                &CrawlConfig {
                    mode: FetchMode::Fast,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(http_calls.load(Ordering::SeqCst), 1);
        assert_eq!(browser_calls.load(Ordering::SeqCst), 0);
    }

    #[cfg(feature = "http")]
    #[tokio::test]
    async fn http_fetcher_descarga_de_un_servidor_local() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            if let Ok((mut s, _)) = listener.accept() {
                let mut buf = [0u8; 1024];
                let _ = s.read(&mut buf);
                let body = "<html><body><article><h1>Rápido</h1><p>Servido por HTTP \
                    puro sin navegador, con texto suficiente.</p></article></body></html>";
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\
                     Connection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = s.write_all(resp.as_bytes());
            }
        });

        let fetcher = Arc::new(crate::HttpFetcher::new().unwrap());
        let crawler = Crawler::new(fetcher);
        let r = crawler
            .crawl(&format!("http://{addr}/"), &CrawlConfig::default())
            .await
            .unwrap();
        assert_eq!(r.status, Some(200));
        assert!(r.markdown.contains("# Rápido"));
    }

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
