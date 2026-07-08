//! Firma estructural del DOM (para caché predictiva por plantilla).
//!
//! Dos páginas generadas por la misma plantilla (p. ej. dos fichas de
//! producto) comparten la secuencia de etiquetas aunque su texto difiera.
//! [`dom_signature`] captura esa estructura en un `u64`: hashea la secuencia
//! de nombres de etiqueta en preorden, ignorando texto y atributos.

use std::hash::{Hash, Hasher};

use ego_tree::NodeRef;
use scraper::{Html, Node};

use crate::cleaner;

/// Calcula una firma estructural del DOM, independiente del contenido textual.
pub fn dom_signature(html: &str) -> u64 {
    let document = Html::parse_document(html);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_structure(document.tree.root(), &mut hasher);
    hasher.finish()
}

fn hash_structure(node: NodeRef<Node>, hasher: &mut impl Hasher) {
    for child in node.children() {
        match child.value() {
            Node::Element(el) => {
                let tag = el.name();
                if cleaner::is_stripped(tag) {
                    continue;
                }
                tag.hash(hasher);
                // Marcadores de apertura/cierre para no confundir hermanos
                // con anidamiento.
                b'<'.hash(hasher);
                hash_structure(child, hasher);
                b'>'.hash(hasher);
            }
            // El texto no participa en la firma estructural.
            _ => hash_structure(child, hasher),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misma_plantilla_distinto_contenido_comparte_firma() {
        let a = r#"<div class="card"><h2>Teclado</h2><span>49.99</span></div>"#;
        let b = r#"<div class="card"><h2>Ratón</h2><span>19.99</span></div>"#;
        assert_eq!(dom_signature(a), dom_signature(b));
    }

    #[test]
    fn estructura_distinta_cambia_la_firma() {
        let a = r#"<div><h2>x</h2><span>y</span></div>"#;
        let b = r#"<div><h2>x</h2><p>y</p></div>"#;
        assert_ne!(dom_signature(a), dom_signature(b));
    }
}
