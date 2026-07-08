//! # crawl4rs-stealth
//!
//! Anti-detección frente a WAFs (Cloudflare, Akamai, …). Define la
//! configuración y el catálogo de perfiles; la integración con CDP
//! endurecido (`chaser-oxide`) y la rotación real de fingerprints llegan en
//! la Fase 5 de la hoja de ruta.

use serde::{Deserialize, Serialize};

/// Un fingerprint de navegador a emular.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    /// Cabecera User-Agent.
    pub user_agent: String,
    /// Cabecera Accept-Language.
    pub accept_language: String,
    /// Vendor reportado por WebGL.
    pub webgl_vendor: String,
    /// Resolución de pantalla (ancho, alto).
    pub viewport: (u32, u32),
}

/// Configuración del modo stealth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Rotar el fingerprint entre peticiones.
    pub rotate_fingerprint: bool,
    /// Introducir retardos variables para emular comportamiento humano.
    pub humanize_timing: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            rotate_fingerprint: true,
            humanize_timing: true,
        }
    }
}

/// Devuelve un conjunto base de fingerprints realistas para rotar.
pub fn default_fingerprints() -> Vec<Fingerprint> {
    vec![
        Fingerprint {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                         (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
                .into(),
            accept_language: "en-US,en;q=0.9".into(),
            webgl_vendor: "Google Inc. (Intel)".into(),
            viewport: (1920, 1080),
        },
        Fingerprint {
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                         (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
                .into(),
            accept_language: "es-ES,es;q=0.9".into(),
            webgl_vendor: "Apple Inc.".into(),
            viewport: (1440, 900),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hay_fingerprints_por_defecto() {
        assert!(!default_fingerprints().is_empty());
        assert!(StealthConfig::default().rotate_fingerprint);
    }
}
