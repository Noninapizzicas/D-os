//! Extracción de texto de PDF **digital** (feature `pdf`).
//!
//! Usa `pdf-extract` (Rust puro, sin dependencia nativa pesada), respetando el
//! ethos de binario ligero. Cubre PDFs con capa de texto — donde el OCR de
//! facturas falla. Los PDFs **escaneados** (imagen sin texto) necesitan OCR:
//! es una fase aparte (dependencia grande) y NO se implementa aquí; para ellos
//! esta función devuelve texto vacío, no un error.

use crate::error::{Error, Result};

/// Convierte los bytes de un PDF a Markdown (texto plano en bloques).
///
/// El resultado es el texto extraído, normalizado a párrafos. No intenta
/// reconstruir tablas ni estilos: es "texto listo para LLM", que es lo que el
/// pipeline persigue.
pub fn pdf_to_markdown(bytes: &[u8]) -> Result<String> {
    let text = pdf_extract::extract_text_from_mem(bytes)
        .map_err(|e| Error::Markdown(format!("no se pudo extraer texto del PDF: {e}")))?;
    Ok(normalize(&text))
}

/// Colapsa espacios/saltos redundantes y recorta.
fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut blank = 0usize;
    for line in text.lines() {
        let t = line.trim_end();
        if t.trim().is_empty() {
            blank += 1;
            if blank <= 1 {
                out.push('\n');
            }
        } else {
            blank = 0;
            out.push_str(t);
            out.push('\n');
        }
    }
    out.trim().to_string()
}
