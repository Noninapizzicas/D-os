//! Orquestador principal: une descarga y pipeline de Markdown.

use std::sync::Arc;

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
}

impl Crawler {
    /// Crea un crawler con el fetcher indicado.
    pub fn new(fetcher: Arc<dyn Fetcher>) -> Self {
        Self {
            fetcher,
            pipeline: MarkdownPipeline::new(),
        }
    }

    /// Descarga y procesa una única URL.
    #[instrument(skip(self), fields(url = %url))]
    pub async fn crawl(&self, url: &str, config: &CrawlConfig) -> Result<CrawlResult> {
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

    /// Descarga y procesa varias URLs de forma secuencial.
    ///
    /// La concurrencia masiva (con `tokio`) llega en la Fase 4; esta versión
    /// mantiene la semántica simple y determinista para los tests.
    pub async fn crawl_many(
        &self,
        urls: &[String],
        config: &CrawlConfig,
    ) -> Vec<Result<CrawlResult>> {
        let mut out = Vec::with_capacity(urls.len());
        for url in urls {
            out.push(self.crawl(url, config).await);
        }
        out
    }
}
