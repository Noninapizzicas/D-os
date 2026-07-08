//! Orquestador principal: une descarga y pipeline de Markdown.

use std::sync::Arc;

use futures::stream::{self, StreamExt};
use tracing::{debug, instrument};

use crawl4rs_markdown::{MarkdownPipeline, PipelineOptions};

use crate::config::{CrawlConfig, FetchMode};
use crate::error::Result;
use crate::fetch::{FetchedPage, Fetcher};
use crate::result::CrawlResult;

/// El crawler: coordina el flujo `fetch → limpiar → markdown → fit`.
///
/// Es genérico sobre el [`Fetcher`], de modo que el mismo orquestador sirve
/// para navegador headless, HTTP plano o HTML estático en tests.
///
/// Puede llevar dos fetchers (HTTP rápido y navegador) para honrar
/// [`FetchMode`]: `fast` usa HTTP, `browser` usa el navegador, y `auto`
/// intenta HTTP y sólo escala al navegador ante un 403/challenge.
#[derive(Clone)]
pub struct Crawler {
    fetcher: Arc<dyn Fetcher>,
    http_fetcher: Option<Arc<dyn Fetcher>>,
    browser_fetcher: Option<Arc<dyn Fetcher>>,
    pipeline: MarkdownPipeline,
    #[cfg(feature = "cache")]
    cache: Option<crate::cache::ResultCache>,
    #[cfg(feature = "extract")]
    extraction: Option<Arc<dyn crawl4rs_extract::ExtractionStrategy>>,
}

impl Crawler {
    /// Crea un crawler con un único fetcher, usado para todos los modos
    /// (comportamiento retrocompatible).
    pub fn new(fetcher: Arc<dyn Fetcher>) -> Self {
        Self {
            fetcher,
            http_fetcher: None,
            browser_fetcher: None,
            pipeline: MarkdownPipeline::new(),
            #[cfg(feature = "cache")]
            cache: None,
            #[cfg(feature = "extract")]
            extraction: None,
        }
    }

    /// Asocia el fetcher HTTP rápido (usado en modo `fast` y como primer
    /// intento en `auto`).
    pub fn with_http_fetcher(mut self, fetcher: Arc<dyn Fetcher>) -> Self {
        self.http_fetcher = Some(fetcher);
        self
    }

    /// Asocia el fetcher de navegador (usado en modo `browser` y como escalada
    /// en `auto`).
    pub fn with_browser_fetcher(mut self, fetcher: Arc<dyn Fetcher>) -> Self {
        self.browser_fetcher = Some(fetcher);
        self
    }

    /// Asocia una estrategia de extracción; su salida se guarda en
    /// [`CrawlResult::extracted`] de cada página.
    #[cfg(feature = "extract")]
    pub fn with_extraction(
        mut self,
        strategy: Arc<dyn crawl4rs_extract::ExtractionStrategy>,
    ) -> Self {
        self.extraction = Some(strategy);
        self
    }

    /// Asocia una caché de resultados; las páginas ya vistas no se vuelven a
    /// descargar ni a procesar.
    #[cfg(feature = "cache")]
    pub fn with_cache(mut self, cache: crate::cache::ResultCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Descarga y procesa una única URL.
    #[instrument(skip(self, config), fields(url = %url))]
    pub async fn crawl(&self, url: &str, config: &CrawlConfig) -> Result<CrawlResult> {
        #[cfg(feature = "cache")]
        if let Some(cache) = &self.cache {
            if let Some(hit) = cache.get(url) {
                debug!("acierto de caché");
                return Ok(hit);
            }
        }

        let result = self.crawl_uncached(url, config).await?;

        #[cfg(feature = "cache")]
        if let Some(cache) = &self.cache {
            cache.put(url, &result);
        }

        Ok(result)
    }

    /// Elige el fetcher y descarga según [`FetchMode`], con escalada en `auto`.
    async fn fetch_by_mode(&self, url: &str, mode: FetchMode) -> Result<FetchedPage> {
        // Sin fetchers específicos, se usa el único fetcher (retrocompat).
        if self.http_fetcher.is_none() && self.browser_fetcher.is_none() {
            return self.fetcher.fetch(url).await;
        }
        let http = self.http_fetcher.as_ref().unwrap_or(&self.fetcher);
        let browser = self.browser_fetcher.as_ref().unwrap_or(&self.fetcher);

        match mode {
            FetchMode::Fast => http.fetch(url).await,
            FetchMode::Browser => browser.fetch(url).await,
            FetchMode::Auto => {
                let page = http.fetch(url).await?;
                if self.browser_fetcher.is_some() && looks_like_challenge(&page) {
                    debug!(status = ?page.status, "HTTP topó con challenge; escalando a navegador");
                    return browser.fetch(url).await;
                }
                Ok(page)
            }
        }
    }

    /// Descarga y procesa una URL sin consultar ni escribir la caché.
    async fn crawl_uncached(&self, url: &str, config: &CrawlConfig) -> Result<CrawlResult> {
        let page = self.fetch_by_mode(url, config.mode).await?;
        debug!(status = ?page.status, bytes = page.html.len(), "página descargada");

        let opts = PipelineOptions {
            query: config.query.clone(),
            word_count_threshold: config.word_count_threshold,
            exclude_external_links: config.exclude_external_links,
        };
        let output = self
            .pipeline
            .run(&page.html, &opts)
            .map_err(|e| crate::error::Error::Markdown(e.to_string()))?;

        #[cfg(feature = "extract")]
        let extracted = match &self.extraction {
            Some(strategy) => match strategy.extract(&page.html, &output.markdown).await {
                Ok(value) => Some(value),
                Err(e) => {
                    tracing::warn!(error = %e, "la extracción falló; se omite");
                    None
                }
            },
            None => None,
        };
        #[cfg(not(feature = "extract"))]
        let extracted = None;

        #[cfg(feature = "extract")]
        let jsonld = if config.extract_jsonld {
            crawl4rs_extract::extract_jsonld(&page.html)
        } else {
            Vec::new()
        };
        #[cfg(not(feature = "extract"))]
        let jsonld = Vec::new();

        Ok(CrawlResult {
            url: page.url,
            status: page.status,
            html: page.html,
            markdown: output.markdown,
            fit_markdown: output.fit_markdown,
            extracted,
            jsonld,
            links: output.links,
        })
    }

    /// Devuelve los enlaces de una página (resueltos a absolutos, únicos, sólo
    /// http/https). Ligero: descarga y extrae enlaces, sin producir Markdown de
    /// contenido. Base de `POST /map`.
    pub async fn map(&self, url: &str, config: &CrawlConfig) -> Result<Vec<String>> {
        use std::collections::HashSet;
        use url::Url;

        let page = self.fetch_by_mode(url, config.mode).await?;
        let converted = crawl4rs_markdown::html_to_markdown(&page.html);
        let base = Url::parse(&page.url).ok();

        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for link in converted.links {
            let abs = match &base {
                Some(b) => b.join(&link).ok(),
                None => Url::parse(&link).ok(),
            };
            if let Some(mut u) = abs {
                if !matches!(u.scheme(), "http" | "https") {
                    continue;
                }
                u.set_fragment(None);
                let s: String = u.into();
                if seen.insert(s.clone()) {
                    out.push(s);
                }
            }
        }
        Ok(out)
    }

    /// Descarga y procesa varias URLs con concurrencia acotada.
    ///
    /// El número de descargas simultáneas es `config.concurrency` (mínimo 1);
    /// el orden de salida se corresponde con el de entrada.
    pub async fn crawl_many(
        &self,
        urls: &[String],
        config: &CrawlConfig,
    ) -> Vec<Result<CrawlResult>> {
        let concurrency = config.concurrency.max(1);
        stream::iter(urls.iter())
            .map(|url| async move { self.crawl(url, config).await })
            .buffered(concurrency)
            .collect()
            .await
    }
}

/// Heurística: ¿la respuesta HTTP parece un muro anti-bot (Cloudflare, etc.)?
fn looks_like_challenge(page: &FetchedPage) -> bool {
    if matches!(page.status, Some(403) | Some(429) | Some(503)) {
        return true;
    }
    let h = page.html.to_ascii_lowercase();
    [
        "just a moment",
        "cf-chl",
        "_cf_chl",
        "challenge-platform",
        "checking your browser",
        "attention required",
        "enable javascript and cookies",
    ]
    .iter()
    .any(|m| h.contains(m))
}
