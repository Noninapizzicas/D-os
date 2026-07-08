//! # crawl4rs-markdown
//!
//! Pipeline de conversión de HTML a Markdown limpio y a `fit_markdown`
//! (la versión filtrada, lista para LLMs).
//!
//! ```
//! use crawl4rs_markdown::{MarkdownPipeline, PipelineOptions};
//!
//! let html = "<article><h1>Hola</h1><p>Un texto de ejemplo.</p></article>";
//! let out = MarkdownPipeline::new()
//!     .run(html, &PipelineOptions { word_count_threshold: 1, ..Default::default() })
//!     .unwrap();
//! assert!(out.markdown.contains("# Hola"));
//! ```

pub mod cleaner;
pub mod converter;
pub mod filter;
pub mod pipeline;
pub mod signature;

pub use converter::{html_to_markdown, Converted};
pub use filter::{Bm25Filter, ContentFilter, PruningFilter};
pub use pipeline::{MarkdownPipeline, PipelineError, PipelineOptions, PipelineOutput};
pub use signature::dom_signature;
