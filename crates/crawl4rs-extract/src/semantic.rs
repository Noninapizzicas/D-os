//! Extracción por densidad semántica.
//!
//! Heurística ligera (inspirada en Readability): elige el bloque de bloque
//! con mayor "densidad de texto" — más texto frente a marcado — como
//! contenido principal del artículo.

use async_trait::async_trait;
use scraper::{Html, Selector};
use serde_json::{json, Value};

use crate::{ExtractionStrategy, Result};

/// Aísla el contenido principal de la página por densidad de texto.
#[derive(Debug, Clone, Default)]
pub struct SemanticDensityStrategy {
    _private: (),
}

impl SemanticDensityStrategy {
    /// Crea la estrategia.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ExtractionStrategy for SemanticDensityStrategy {
    fn name(&self) -> &'static str {
        "semantic-density"
    }

    async fn extract(&self, html: &str, _markdown: &str) -> Result<Value> {
        let doc = Html::parse_document(html);
        // Candidatos habituales para el cuerpo del artículo.
        let selector = Selector::parse("article, main, section, div").unwrap();

        let mut best_text = String::new();
        let mut best_score = 0.0f32;

        for el in doc.select(&selector) {
            let text = el.text().collect::<String>();
            let text_len = text.split_whitespace().count() as f32;
            if text_len < 25.0 {
                continue;
            }
            // Densidad = palabras de texto / (1 + número de elementos hijos).
            let child_els = el.children().filter(|c| c.value().is_element()).count() as f32;
            let density = text_len / (1.0 + child_els);
            let score = density * text_len.sqrt();
            if score > best_score {
                best_score = score;
                best_text = text.split_whitespace().collect::<Vec<_>>().join(" ");
            }
        }

        Ok(json!({
            "content": best_text,
            "score": best_score,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn elige_el_bloque_mas_denso() {
        let html = r#"
            <body>
                <nav>menú corto</nav>
                <article>
                    Este es el cuerpo principal del artículo, con muchas
                    palabras de contenido real que superan holgadamente el
                    umbral mínimo necesario para ser considerado contenido
                    principal por la heurística de densidad semántica.
                </article>
                <footer>pie</footer>
            </body>"#;
        let out = SemanticDensityStrategy::new()
            .extract(html, "")
            .await
            .unwrap();
        let content = out["content"].as_str().unwrap();
        assert!(content.contains("cuerpo principal"));
        assert!(!content.contains("menú"));
    }
}
