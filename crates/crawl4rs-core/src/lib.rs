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
#[cfg(feature = "pdf")]
pub mod pdf;
#[cfg(feature = "playwright")]
pub mod playwright;
pub mod result;
pub mod session;

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
#[cfg(feature = "playwright")]
pub use playwright::PlaywrightFetcher;
pub use result::CrawlResult;
pub use session::Session;

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
                ..Default::default()
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

    /// Fetcher que exige sesión: sin ella devuelve 401; con ella, el contenido.
    struct SessionGatedFetcher {
        cell: crate::session::SessionCell,
        html: String,
        calls: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl Fetcher for SessionGatedFetcher {
        async fn fetch(&self, url: &str) -> Result<FetchedPage> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let tiene = self.cell.read().map(|g| g.is_some()).unwrap_or(false);
            Ok(FetchedPage {
                url: url.to_string(),
                status: Some(if tiene { 200 } else { 401 }),
                html: if tiene {
                    self.html.clone()
                } else {
                    "<html><body>necesitas iniciar sesión</body></html>".into()
                },
                ..Default::default()
            })
        }
    }

    /// Autenticador de prueba: cuenta los logins y devuelve una sesión con cookie.
    struct FakeAuth {
        logins: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl crate::session::Authenticator for FakeAuth {
        async fn login(&self) -> Result<crate::session::Session> {
            self.logins.fetch_add(1, Ordering::SeqCst);
            Ok(crate::session::Session::from_storage_state(
                serde_json::json!({ "cookies": [{ "name": "sid", "value": "ok" }] }),
            ))
        }
    }

    const OK_HTML: &str = "<html><body><article><h1>Contenido</h1><p>Un párrafo \
        con suficientes palabras para superar el umbral por defecto del pipeline.</p>\
        </article></body></html>";

    #[tokio::test]
    async fn auto_reloguea_al_perder_sesion_y_reintenta() {
        // Sin sesión → 401; el crawler reloguea, refresca la celda y reintenta.
        let cell = crate::session::empty_session_cell();
        let calls = Arc::new(AtomicUsize::new(0));
        let gated = Arc::new(SessionGatedFetcher {
            cell: cell.clone(),
            html: OK_HTML.to_string(),
            calls: calls.clone(),
        });
        let logins = Arc::new(AtomicUsize::new(0));
        let auth = Arc::new(FakeAuth {
            logins: logins.clone(),
        });

        let crawler = Crawler::new(gated.clone())
            .with_http_fetcher(gated.clone())
            .with_auto_login(auth, cell);

        let r = crawler
            .crawl(
                "https://x.test/panel",
                &CrawlConfig {
                    mode: FetchMode::Auto,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(
            r.markdown.contains("Contenido"),
            "tras el re-login ve el contenido"
        );
        assert_eq!(logins.load(Ordering::SeqCst), 1, "un solo login");
        assert_eq!(calls.load(Ordering::SeqCst), 2, "fetch: 401 y luego 200");
    }

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

    /// Construye un PDF de una página con `text`, con xref válido.
    #[cfg(feature = "pdf")]
    fn make_pdf(text: &str) -> Vec<u8> {
        let content = format!("BT /F1 24 Tf 72 720 Td ({text}) Tj ET");
        let objs = [
            "<</Type/Catalog/Pages 2 0 R>>".to_string(),
            "<</Type/Pages/Kids[3 0 R]/Count 1>>".to_string(),
            "<</Type/Page/Parent 2 0 R/MediaBox[0 0 612 792]/Contents 4 0 R\
             /Resources<</Font<</F1 5 0 R>>>>>>"
                .to_string(),
            format!(
                "<</Length {}>>\nstream\n{content}\nendstream",
                content.len()
            ),
            "<</Type/Font/Subtype/Type1/BaseFont/Helvetica>>".to_string(),
        ];
        let mut pdf = String::from("%PDF-1.4\n");
        let mut offsets = Vec::new();
        for (i, o) in objs.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.push_str(&format!("{} 0 obj\n{o}\nendobj\n", i + 1));
        }
        let xref_start = pdf.len();
        pdf.push_str(&format!(
            "xref\n0 {}\n0000000000 65535 f \n",
            objs.len() + 1
        ));
        for off in &offsets {
            pdf.push_str(&format!("{off:010} 00000 n \n"));
        }
        pdf.push_str(&format!(
            "trailer\n<</Size {}/Root 1 0 R>>\nstartxref\n{xref_start}\n%%EOF",
            objs.len() + 1
        ));
        pdf.into_bytes()
    }

    #[cfg(feature = "pdf")]
    #[tokio::test]
    async fn pdf_digital_se_convierte_a_markdown() {
        let bytes = make_pdf("Hola PDF");
        let text = crate::pdf::pdf_to_markdown(&bytes).expect("extrae texto del PDF");
        assert!(text.contains("Hola PDF"), "texto: {text:?}");
    }

    #[cfg(all(feature = "pdf", feature = "http"))]
    #[tokio::test]
    async fn crawl_detecta_pdf_por_url_y_extrae() {
        use std::io::{Read, Write};
        let pdf = make_pdf("Factura 42");
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            if let Ok((mut s, _)) = listener.accept() {
                let mut buf = [0u8; 1024];
                let _ = s.read(&mut buf);
                let mut resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n",
                    pdf.len()
                )
                .into_bytes();
                resp.extend_from_slice(&pdf);
                let _ = s.write_all(&resp);
            }
        });

        let crawler = Crawler::new(Arc::new(crate::HttpFetcher::new().unwrap()));
        let r = crawler
            .crawl(&format!("http://{addr}/doc.pdf"), &CrawlConfig::default())
            .await
            .unwrap();
        assert!(
            r.markdown.contains("Factura 42"),
            "markdown: {}",
            r.markdown
        );
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
