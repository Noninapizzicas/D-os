//! Limpieza de HTML: elimina elementos que no aportan contenido.

/// Etiquetas cuyo contenido se descarta por completo antes de convertir.
pub const STRIP_TAGS: &[&str] = &[
    "script", "style", "noscript", "template", "svg", "iframe", "canvas", "head",
];

/// Etiquetas estructurales de "ruido" (navegación, pies, anuncios) que se
/// omiten al generar Markdown pero cuyo texto podría rescatarse en el futuro.
pub const NOISE_TAGS: &[&str] = &["nav", "footer", "aside", "form"];

/// Indica si una etiqueta debe eliminarse por completo.
pub fn is_stripped(tag: &str) -> bool {
    STRIP_TAGS.contains(&tag)
}

/// Indica si una etiqueta se considera ruido estructural.
pub fn is_noise(tag: &str) -> bool {
    NOISE_TAGS.contains(&tag)
}
