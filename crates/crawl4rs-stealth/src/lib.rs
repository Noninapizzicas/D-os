//! # crawl4rs-stealth
//!
//! Anti-detección frente a WAFs (Cloudflare, Akamai, …). Provee:
//!
//! - [`Fingerprint`]: un perfil de navegador a emular (UA, idioma, WebGL…).
//! - [`StealthConfig`]: qué técnicas activar.
//! - [`StealthEngine`]: rota fingerprints, genera el script de ocultación que
//!   se inyecta antes de cargar la página y produce retardos "humanos".
//!
//! El motor es determinista (sin `rand`): la rotación es round-robin y los
//! retardos provienen de un xorshift sembrado, de modo que los tests son
//! reproducibles. `crawl4rs-core` lo aplica sobre Chromium vía CDP.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Un fingerprint de navegador a emular.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    /// Cabecera User-Agent.
    pub user_agent: String,
    /// Cabecera Accept-Language.
    pub accept_language: String,
    /// `navigator.platform` a reportar.
    pub platform: String,
    /// Vendor reportado por WebGL.
    pub webgl_vendor: String,
    /// Renderer reportado por WebGL.
    pub webgl_renderer: String,
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
    /// Retardo humano mínimo (ms).
    pub min_delay_ms: u64,
    /// Retardo humano máximo (ms).
    pub max_delay_ms: u64,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            rotate_fingerprint: true,
            humanize_timing: true,
            min_delay_ms: 150,
            max_delay_ms: 900,
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
            platform: "Win32".into(),
            webgl_vendor: "Google Inc. (Intel)".into(),
            webgl_renderer: "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)"
                .into(),
            viewport: (1920, 1080),
        },
        Fingerprint {
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                         (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
                .into(),
            accept_language: "es-ES,es;q=0.9,en;q=0.8".into(),
            platform: "MacIntel".into(),
            webgl_vendor: "Apple Inc.".into(),
            webgl_renderer: "Apple GPU".into(),
            viewport: (1440, 900),
        },
        Fingerprint {
            user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
                         (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
                .into(),
            accept_language: "en-GB,en;q=0.9".into(),
            platform: "Linux x86_64".into(),
            webgl_vendor: "Google Inc. (NVIDIA)".into(),
            webgl_renderer:
                "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0, D3D11)".into(),
            viewport: (1600, 900),
        },
    ]
}

/// Escapa una cadena para incrustarla en un literal JS entre comillas dobles.
fn js_string(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

/// Genera el script de ocultación a inyectar **antes** de cargar la página.
///
/// Neutraliza las señales típicas que delatan a un navegador automatizado:
/// `navigator.webdriver`, ausencia de plugins, idiomas, vendor de WebGL y el
/// objeto `window.chrome`.
pub fn stealth_script(fp: &Fingerprint) -> String {
    let langs: Vec<String> = fp
        .accept_language
        .split(',')
        .map(|l| l.split(';').next().unwrap_or(l).trim())
        .filter(|l| !l.is_empty())
        .map(js_string)
        .collect();
    let langs = langs.join(", ");
    let vendor = js_string(&fp.webgl_vendor);
    let renderer = js_string(&fp.webgl_renderer);
    let platform = js_string(&fp.platform);

    format!(
        r#"(() => {{
  Object.defineProperty(navigator, 'webdriver', {{ get: () => undefined }});
  Object.defineProperty(navigator, 'languages', {{ get: () => [{langs}] }});
  Object.defineProperty(navigator, 'platform', {{ get: () => {platform} }});
  Object.defineProperty(navigator, 'plugins', {{ get: () => [1, 2, 3, 4, 5] }});
  window.chrome = window.chrome || {{ runtime: {{}} }};
  const origQuery = window.navigator.permissions && window.navigator.permissions.query;
  if (origQuery) {{
    window.navigator.permissions.query = (p) =>
      p && p.name === 'notifications'
        ? Promise.resolve({{ state: Notification.permission }})
        : origQuery(p);
  }}
  const getParam = WebGLRenderingContext.prototype.getParameter;
  WebGLRenderingContext.prototype.getParameter = function (p) {{
    if (p === 37445) return {vendor};
    if (p === 37446) return {renderer};
    return getParam.apply(this, [p]);
  }};
}})();"#
    )
}

/// Motor stealth: rota fingerprints, genera scripts y retardos humanos.
pub struct StealthEngine {
    fingerprints: Vec<Fingerprint>,
    config: StealthConfig,
    counter: AtomicUsize,
    rng: Mutex<u64>,
}

impl std::fmt::Debug for StealthEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StealthEngine")
            .field("fingerprints", &self.fingerprints.len())
            .field("config", &self.config)
            .finish()
    }
}

impl StealthEngine {
    /// Crea el motor con los fingerprints por defecto.
    pub fn new(config: StealthConfig) -> Self {
        Self::with_fingerprints(config, default_fingerprints())
    }

    /// Crea el motor con un conjunto de fingerprints propio.
    ///
    /// Si `fingerprints` está vacío, se usa el conjunto por defecto.
    pub fn with_fingerprints(config: StealthConfig, fingerprints: Vec<Fingerprint>) -> Self {
        let fingerprints = if fingerprints.is_empty() {
            default_fingerprints()
        } else {
            fingerprints
        };
        Self {
            fingerprints,
            config,
            counter: AtomicUsize::new(0),
            // Semilla fija: comportamiento variado pero reproducible.
            rng: Mutex::new(0x2545_F491_4F6C_DD1D),
        }
    }

    /// Configuración activa.
    pub fn config(&self) -> &StealthConfig {
        &self.config
    }

    /// Devuelve el siguiente fingerprint (round-robin si la rotación está
    /// activada; siempre el primero si no).
    pub fn next_fingerprint(&self) -> Fingerprint {
        let idx = if self.config.rotate_fingerprint {
            self.counter.fetch_add(1, Ordering::Relaxed) % self.fingerprints.len()
        } else {
            0
        };
        self.fingerprints[idx].clone()
    }

    /// Script de ocultación para un fingerprint dado.
    pub fn script_for(&self, fp: &Fingerprint) -> String {
        stealth_script(fp)
    }

    /// Siguiente retardo "humano". `Duration::ZERO` si `humanize_timing` está
    /// desactivado o el rango es vacío.
    pub fn next_delay(&self) -> Duration {
        if !self.config.humanize_timing {
            return Duration::ZERO;
        }
        let (lo, hi) = (self.config.min_delay_ms, self.config.max_delay_ms);
        if hi <= lo {
            return Duration::from_millis(lo);
        }
        let mut state = self.rng.lock().unwrap();
        // xorshift64.
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        Duration::from_millis(lo + x % (hi - lo))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotacion_recorre_todos_los_fingerprints() {
        let engine = StealthEngine::new(StealthConfig::default());
        let n = default_fingerprints().len();
        let uas: std::collections::HashSet<_> = (0..n)
            .map(|_| engine.next_fingerprint().user_agent)
            .collect();
        assert_eq!(uas.len(), n, "la rotación debe cubrir todos los perfiles");
    }

    #[test]
    fn sin_rotacion_siempre_el_mismo() {
        let engine = StealthEngine::new(StealthConfig {
            rotate_fingerprint: false,
            ..Default::default()
        });
        let a = engine.next_fingerprint().user_agent;
        let b = engine.next_fingerprint().user_agent;
        assert_eq!(a, b);
    }

    #[test]
    fn retardo_dentro_del_rango() {
        let engine = StealthEngine::new(StealthConfig {
            min_delay_ms: 100,
            max_delay_ms: 200,
            ..Default::default()
        });
        for _ in 0..50 {
            let d = engine.next_delay().as_millis() as u64;
            assert!((100..200).contains(&d), "retardo fuera de rango: {d}");
        }
    }

    #[test]
    fn script_incluye_las_neutralizaciones_clave() {
        let fp = &default_fingerprints()[0];
        let script = stealth_script(fp);
        assert!(script.contains("navigator, 'webdriver'"));
        assert!(script.contains("getParameter"));
        assert!(script.contains(&fp.webgl_vendor));
        assert!(script.contains("es-ES") || script.contains("en-US"));
    }
}
