//! Filtros de contenido para generar `fit_markdown`.
//!
//! Un [`ContentFilter`] recibe los bloques de Markdown (separados por línea
//! en blanco) y devuelve el subconjunto relevante.

/// Divide un documento Markdown en bloques (párrafos, encabezados, ítems).
pub fn split_blocks(markdown: &str) -> Vec<String> {
    markdown
        .split("\n\n")
        .map(|b| b.trim().to_string())
        .filter(|b| !b.is_empty())
        .collect()
}

/// Une bloques de nuevo en un documento Markdown.
pub fn join_blocks(blocks: &[String]) -> String {
    blocks.join("\n\n")
}

/// Estrategia de filtrado de bloques de contenido.
pub trait ContentFilter {
    /// Nombre legible del filtro (para logs).
    fn name(&self) -> &'static str;

    /// Devuelve los bloques que deben conservarse.
    fn filter(&self, blocks: Vec<String>) -> Vec<String>;
}

/// Poda por densidad de texto: descarta bloques demasiado cortos, salvo que
/// sean estructurales (encabezados, ítems de lista, citas, código).
#[derive(Debug, Clone)]
pub struct PruningFilter {
    /// Mínimo de palabras para conservar un párrafo normal.
    pub word_count_threshold: usize,
}

impl PruningFilter {
    /// Crea un filtro de poda con el umbral dado.
    pub fn new(word_count_threshold: usize) -> Self {
        Self {
            word_count_threshold,
        }
    }

    fn is_structural(block: &str) -> bool {
        let t = block.trim_start();
        t.starts_with('#')
            || t.starts_with("- ")
            || t.starts_with("> ")
            || t.starts_with("```")
            || t.starts_with("![")
            || t.chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
                && t.contains(". ")
    }
}

impl ContentFilter for PruningFilter {
    fn name(&self) -> &'static str {
        "pruning"
    }

    fn filter(&self, blocks: Vec<String>) -> Vec<String> {
        blocks
            .into_iter()
            .filter(|b| Self::is_structural(b) || word_count(b) >= self.word_count_threshold)
            .collect()
    }
}

/// Filtro por relevancia BM25 respecto a una consulta.
///
/// Implementación autónoma del clásico Okapi BM25 (k1 = 1.2, b = 0.75). Cada
/// bloque es un "documento"; se conservan los que superan `score_threshold`,
/// preservando el orden original.
#[derive(Debug, Clone)]
pub struct Bm25Filter {
    query_terms: Vec<String>,
    score_threshold: f32,
    k1: f32,
    b: f32,
}

impl Bm25Filter {
    /// Crea un filtro BM25 para la consulta dada.
    pub fn new(query: &str, score_threshold: f32) -> Self {
        Self {
            query_terms: tokenize(query),
            score_threshold,
            k1: 1.2,
            b: 0.75,
        }
    }

    /// Puntúa cada bloque; expuesto para inspección/tests.
    pub fn scores(&self, blocks: &[String]) -> Vec<f32> {
        if self.query_terms.is_empty() || blocks.is_empty() {
            return vec![0.0; blocks.len()];
        }

        let docs: Vec<Vec<String>> = blocks.iter().map(|b| tokenize(b)).collect();
        let n = docs.len() as f32;
        let avgdl = docs.iter().map(|d| d.len()).sum::<usize>() as f32 / n;

        // Frecuencia de documento por término de la consulta.
        let mut df = std::collections::HashMap::new();
        for term in &self.query_terms {
            let count = docs.iter().filter(|d| d.contains(term)).count();
            df.insert(term.clone(), count);
        }

        docs.iter()
            .map(|doc| {
                let dl = doc.len() as f32;
                let mut score = 0.0f32;
                for term in &self.query_terms {
                    let f = doc.iter().filter(|w| *w == term).count() as f32;
                    if f == 0.0 {
                        continue;
                    }
                    let n_q = *df.get(term).unwrap_or(&0) as f32;
                    // idf con suavizado (siempre positivo).
                    let idf = ((n - n_q + 0.5) / (n_q + 0.5) + 1.0).ln();
                    let denom = f + self.k1 * (1.0 - self.b + self.b * dl / avgdl);
                    score += idf * (f * (self.k1 + 1.0)) / denom;
                }
                score
            })
            .collect()
    }
}

impl ContentFilter for Bm25Filter {
    fn name(&self) -> &'static str {
        "bm25"
    }

    fn filter(&self, blocks: Vec<String>) -> Vec<String> {
        if self.query_terms.is_empty() {
            return blocks;
        }
        let scores = self.scores(&blocks);
        blocks
            .into_iter()
            .zip(scores)
            .filter(|(_, s)| *s >= self.score_threshold)
            .map(|(b, _)| b)
            .collect()
    }
}

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Tokenización simple: minúsculas, separando por caracteres no alfanuméricos.
fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poda_descarta_bloques_cortos_pero_conserva_encabezados() {
        let f = PruningFilter::new(5);
        let blocks = vec![
            "# Encabezado".to_string(),
            "corto".to_string(),
            "este bloque tiene bastantes palabras y debe conservarse".to_string(),
        ];
        let kept = f.filter(blocks);
        assert_eq!(kept.len(), 2);
        assert!(kept[0].starts_with('#'));
    }

    #[test]
    fn bm25_prioriza_bloques_relevantes() {
        let f = Bm25Filter::new("rust async", 0.01);
        let blocks = vec![
            "Rust ofrece async sin recolector de basura".to_string(),
            "Una receta de cocina totalmente ajena al tema".to_string(),
        ];
        let scores = f.scores(&blocks);
        assert!(scores[0] > scores[1]);
        let kept = f.filter(blocks);
        assert_eq!(kept.len(), 1);
        assert!(kept[0].contains("Rust"));
    }
}
