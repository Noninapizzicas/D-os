//! Autenticación JWT (HS256).

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

/// Claims del token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Sujeto (identificador del cliente).
    pub sub: String,
    /// Expiración (segundos desde época Unix).
    pub exp: usize,
}

/// Configuración de autenticación del servidor.
#[derive(Clone)]
pub struct AuthConfig {
    secret: Arc<String>,
    /// API key requerida para emitir tokens. `None` → emisión abierta (dev).
    api_key: Option<Arc<String>>,
    /// Validez de los tokens emitidos, en segundos.
    ttl_secs: u64,
    /// `false` → auth ABIERTA: el middleware deja pasar sin token. La auth
    /// protege una FRONTERA; cuando no la hay (loopback) o está en otra capa
    /// (Docker publicando solo a 127.0.0.1 del host), exigir token es teatro.
    exigir: bool,
}

impl AuthConfig {
    /// Crea la configuración con un secreto de firma (auth activa).
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: Arc::new(secret.into()),
            api_key: None,
            ttl_secs: 3600,
            exigir: true,
        }
    }

    /// Auth ABIERTA: `/auth/token` sigue emitiendo (compatibilidad con clientes
    /// que hacen el baile del token) pero las rutas protegidas no lo exigen.
    /// El secreto efímero solo firma esos tokens de cortesía.
    pub fn abierta() -> Self {
        let mut cfg = Self::new(uuid::Uuid::new_v4().to_string());
        cfg.exigir = false;
        cfg
    }

    /// ¿El middleware exige token?
    pub fn exige(&self) -> bool {
        self.exigir
    }

    /// Exige una API key concreta para emitir tokens.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(Arc::new(api_key.into()));
        self
    }

    /// Fija la validez de los tokens (segundos).
    pub fn with_ttl_secs(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = ttl_secs;
        self
    }

    /// Comprueba si `presented` autoriza la emisión de un token.
    pub fn api_key_ok(&self, presented: Option<&str>) -> bool {
        match &self.api_key {
            None => true,
            Some(expected) => presented == Some(expected.as_str()),
        }
    }

    /// Emite un JWT para `subject`.
    pub fn issue(&self, subject: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let claims = Claims {
            sub: subject.to_string(),
            exp: (now + self.ttl_secs) as usize,
        };
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    /// Verifica un token y devuelve sus claims.
    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(data.claims)
    }
}

/// Middleware que exige un `Authorization: Bearer <jwt>` válido.
pub async fn require_jwt(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if !state.auth.exige() {
        return Ok(next.run(request).await);
    }
    let header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match header.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t.trim(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if state.auth.verify(token).is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_activa_exige_y_verifica_roundtrip() {
        let auth = AuthConfig::new("secreto-de-test");
        assert!(auth.exige());
        let token = auth.issue("cliente").expect("emite");
        assert_eq!(auth.verify(&token).expect("verifica").sub, "cliente");
    }

    #[test]
    fn auth_abierta_no_exige_pero_sigue_emitiendo() {
        // Compatibilidad: los clientes que hacen el baile del token no se rompen.
        let auth = AuthConfig::abierta();
        assert!(!auth.exige());
        let token = auth.issue("cliente").expect("emite tokens de cortesía");
        assert!(auth.verify(&token).is_ok());
    }

    #[test]
    fn api_key_ok_sin_configurar_es_abierta() {
        let auth = AuthConfig::abierta();
        assert!(auth.api_key_ok(None));
    }
}
