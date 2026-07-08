//! Orquestador principal: une descarga y pipeline de Markdown.

use std::sync::Arc;

use futures::stream::{self, StreamExt};
use tracing::{debug, instrument};

use crawl4rs_markdown::{MarkdownPipeline, PipelineOptions};

use crate::config::CrawlConfig;
use crate::error::Result;
use crate::fetch::Fetcher;
use crate::result::CrawlResult;

/// El crawler: coordina el flujo `fetch → limpiar → markdown → fit`.
///
/// Es genérico sobre el [`Fetcher`], de modo que el mismo orquestador sirve
/// para navegador headless, HTTP plano o HTML estático en tests.
#[derive(Clone)]
pub struct Crawler {
    fetcher: Arc<dyn Fetcher>,
    pipeline: MarkdownPipeline,
    #[cfg(feature = "cache")]
    cache: Option<crate::cache::ResultCache>,
}

impl Crawler {
    /// Crea un crawler con el fetcher indicado.
    pub fn new(fetcher: Arc<dyn Fetcher>) -> Self {
        Self {
            fetcher,
            pipeline: MarkdownPipeline::new(),
            #[cfg(feature = "cache")]
            cache: None,
        }
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

    /// Descarga y procesa una URL sin consultar ni escribir la caché.
    async fn crawl_uncached(&self, url: &str, config: &CrawlConfig) -> Result<CrawlResult> {
        let page = self.fetcher.fetch(url).await?;
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

        Ok(CrawlResult {
            url: page.url,
            status: page.status,
            html: page.html,
            markdown: output.markdown,
            fit_markdown: output.fit_markdown,
            extracted: None,
            links: output.links,
        })
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
