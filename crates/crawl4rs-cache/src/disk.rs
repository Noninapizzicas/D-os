//! Caché persistente en disco, respaldada por `sled`.

use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

/// Errores de la caché en disco.
#[derive(Debug, Error)]
pub enum CacheError {
    /// Error de la base de datos `sled`.
    #[error("error de sled: {0}")]
    Db(#[from] sled::Error),

    /// Error de (de)serialización JSON.
    #[error("error de serialización: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Resultado de operaciones de caché en disco.
pub type Result<T> = std::result::Result<T, CacheError>;

/// Caché persistente clave→valor sobre `sled`. Los valores se serializan a
/// JSON. Es barata de clonar (comparte el handle de la base).
#[derive(Clone)]
pub struct DiskCache {
    db: sled::Db,
}

impl DiskCache {
    /// Abre (o crea) la caché en el directorio indicado.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    /// Recupera y deserializa el valor de una clave, si existe.
    pub fn get<V: DeserializeOwned>(&self, key: &str) -> Result<Option<V>> {
        match self.db.get(key)? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Inserta o actualiza el valor de una clave.
    pub fn put<V: Serialize>(&self, key: &str, value: &V) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        self.db.insert(key, bytes)?;
        Ok(())
    }

    /// Indica si una clave está presente.
    pub fn contains(&self, key: &str) -> Result<bool> {
        Ok(self.db.contains_key(key)?)
    }

    /// Número de entradas almacenadas.
    pub fn len(&self) -> usize {
        self.db.len()
    }

    /// Indica si la caché está vacía.
    pub fn is_empty(&self) -> bool {
        self.db.is_empty()
    }

    /// Fuerza el volcado a disco de los datos pendientes.
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persiste_entre_aperturas() {
        let dir = tempfile::tempdir().unwrap();
        {
            let cache = DiskCache::open(dir.path()).unwrap();
            cache.put("clave", &vec![1u32, 2, 3]).unwrap();
            cache.flush().unwrap();
        }
        // Reabrir desde disco y comprobar que el valor sigue ahí.
        let cache = DiskCache::open(dir.path()).unwrap();
        let v: Option<Vec<u32>> = cache.get("clave").unwrap();
        assert_eq!(v, Some(vec![1, 2, 3]));
        assert!(cache.contains("clave").unwrap());
        assert!(cache.get::<Vec<u32>>("ausente").unwrap().is_none());
    }
}
