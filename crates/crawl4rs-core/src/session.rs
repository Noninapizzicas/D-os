//! Sesión de autenticación reutilizable entre las dos marchas.
//!
//! Tras un login, Playwright entrega su `storageState` (cookies + localStorage).
//! Es el **objeto de intercambio** del puente:
//!
//! - La **marcha larga** (Playwright) lo reutiliza entero — `POST /abrir { sesion }`.
//! - La **marcha corta** (HTTP) solo puede usar las **cookies**: el localStorage
//!   es estado de cliente y no viaja por HTTP. Degradación honesta.

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Celda de sesión **compartida**: los fetchers la leen en cada descarga y el
/// crawler la reescribe tras un re-login. Así el lazo automático refresca la
/// sesión sin reconstruir nada. Se usa un `RwLock` síncrono: el guard nunca se
/// mantiene a través de un `await` (se extrae la cookie y se suelta).
pub type SessionCell = Arc<RwLock<Option<Session>>>;

/// Crea una celda de sesión vacía.
pub fn empty_session_cell() -> SessionCell {
    Arc::new(RwLock::new(None))
}

/// Quien sabe autenticarse y devolver una [`Session`] fresca. El crawler lo
/// invoca cuando detecta que la sesión se perdió (401 / redirección a login).
#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Hace login y devuelve la sesión capturada. El fallo se propaga.
    async fn login(&self) -> Result<Session>;
}

/// Sesión capturada: el `storageState` opaco de Playwright.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Session {
    /// `storageState` de Playwright (`{ cookies: [...], origins: [...] }`).
    #[serde(default)]
    pub storage_state: serde_json::Value,
}

impl Session {
    /// Crea una sesión desde un `storageState` de Playwright.
    pub fn from_storage_state(value: serde_json::Value) -> Self {
        Self {
            storage_state: value,
        }
    }

    /// Cabecera `Cookie: k=v; …` para la marcha corta. `None` si no hay cookies
    /// utilizables. El localStorage se ignora aquí a propósito (no viaja por HTTP).
    pub fn cookie_header(&self) -> Option<String> {
        let cookies = self.storage_state.get("cookies")?.as_array()?;
        let parts: Vec<String> = cookies
            .iter()
            .filter_map(|c| {
                let name = c.get("name")?.as_str()?;
                let value = c.get("value")?.as_str()?;
                Some(format!("{name}={value}"))
            })
            .collect();
        (!parts.is_empty()).then(|| parts.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn cookie_header_junta_las_cookies() {
        let s = Session::from_storage_state(json!({
            "cookies": [
                { "name": "sid", "value": "abc" },
                { "name": "csrf", "value": "xyz" }
            ],
            "origins": []
        }));
        assert_eq!(s.cookie_header().as_deref(), Some("sid=abc; csrf=xyz"));
    }

    #[test]
    fn sin_cookies_no_hay_cabecera() {
        let s = Session::from_storage_state(json!({ "cookies": [], "origins": [] }));
        assert!(s.cookie_header().is_none());
        assert!(Session::default().cookie_header().is_none());
    }
}
