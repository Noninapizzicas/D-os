//! Pipeline de alto nivel: HTML → `markdown` + `fit_markdown`.

use tracing::debug;

use crate::converter::{self};
use crate::filter::{self, Bm25Filter, ContentFilter, PruningFilter};

/// Opciones que controlan una ejecución del pipeline.
#[derive(Debug, Clone, Default)]
pub struct PipelineOptions {
    /// Consulta para el filtro BM25. `None` → sólo poda heurística.
    pub query: Option<String>,
    /// Umbral de palabras para la poda.
    pub word_count_threshold: usize,
    /// Excluir enlaces absolutos (http/https) de la lista de enlaces.
    pub exclude_external_links: bool,
}

/// Salida del pipeline.
#[derive(Debug, Clone)]
pub struct PipelineOutput {
    /// Markdown completo.
    pub markdown: String,
    /// Markdown filtrado ("fit"), listo para un LLM.
    pub fit_markdown: String,
    /// Enlaces encontrados.
    pub links: Vec<String>,
}

/// Errores del pipeline (reservado para fallos futuros de parsing/IO).
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    /// Error genérico de procesamiento.
    #[error("error de pipeline: {0}")]
    Processing(String),
}

/// El pipeline de conversión. Sin estado; reutilizable entre páginas.
#[derive(Debug, Clone, Default)]
pub struct MarkdownPipeline;

impl MarkdownPipeline {
    /// Crea un pipeline nuevo.
    pub fn new() -> Self {
        Self
    }

    /// Ejecuta el pipeline sobre un documento HTML.
    pub fn run(&self, html: &str, opts: &PipelineOptions) -> Result<PipelineOutput, PipelineError> {
        let converted = converter::html_to_markdown(html);
        let markdown = converted.markdown;

        let mut blocks = filter::split_blocks(&markdown);
        debug!(bloques = blocks.len(), "markdown convertido");

        // Paso 1: poda heurística por densidad de texto.
        let pruning = PruningFilter::new(opts.word_count_threshold.max(1));
        blocks = pruning.filter(blocks);

        // Paso 2 (opcional): relevancia BM25 si hay consulta.
        if let Some(query) = opts.query.as_deref().filter(|q| !q.trim().is_empty()) {
            let bm25 = Bm25Filter::new(query, 0.0001);
            blocks = bm25.filter(blocks);
        }

        let fit_markdown = filter::join_blocks(&blocks);

        let links = if opts.exclude_external_links {
            converted
                .links
                .into_iter()
                .filter(|l| !(l.starts_with("http://") || l.starts_with("https://")))
                .collect()
        } else {
            converted.links
        };

        Ok(PipelineOutput {
            markdown,
            fit_markdown,
            links,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML: &str = r#"
        <html><head><style>.x{}</style></head>
        <body>
            <nav><a href="/inicio">Inicio</a></nav>
            <article>
                <h1>Título principal</h1>
                <p>Este es un párrafo largo con suficientes palabras para
                   superar el umbral de poda establecido por defecto.</p>
                <ul><li>Uno</li><li>Dos</li></ul>
                <p><a href="https://externo.test">enlace externo</a></p>
            </article>
            <footer>pie de página irrelevante</footer>
        </body></html>
    "#;

    #[test]
    fn pipeline_produce_markdown_y_fit() {
        let out = MarkdownPipeline::new()
            .run(
                HTML,
                &PipelineOptions {
                    word_count_threshold: 8,
                    ..Default::default()
                },
            )
            .unwrap();

        assert!(out.markdown.contains("# Título principal"));
        assert!(out.markdown.contains("- Uno"));
        // El script/style y el nav/footer no deben aparecer.
        assert!(!out.markdown.contains(".x{}"));
        assert!(!out.markdown.contains("pie de página"));
        // fit_markdown conserva el encabezado y el párrafo largo.
        assert!(out.fit_markdown.contains("Título principal"));
    }

    #[test]
    fn excluir_enlaces_externos() {
        let out = MarkdownPipeline::new()
            .run(
                HTML,
                &PipelineOptions {
                    word_count_threshold: 8,
                    exclude_external_links: true,
                    ..Default::default()
                },
            )
            .unwrap();
        assert!(out.links.iter().all(|l| !l.starts_with("http")));
    }
}
