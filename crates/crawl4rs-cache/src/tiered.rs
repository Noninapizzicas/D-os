//! Caché de dos niveles: LRU en RAM por delante de `sled` en disco.

use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::trace;

use crate::disk::{DiskCache, Result};
use crate::{Cache, MemoryCache};

/// Caché jerárquica: consulta primero la RAM (rápida) y, si falla, el disco
/// (persistente), promocionando el valor a RAM. Las escrituras van a ambos
/// niveles.
pub struct TieredCache<V: Clone + Serialize + DeserializeOwned> {
    memory: MemoryCache<String, V>,
    disk: DiskCache,
}

impl<V: Clone + Serialize + DeserializeOwned> TieredCache<V> {
    /// Crea la caché con una capacidad de RAM y una caché de disco dadas.
    pub fn new(memory_capacity: usize, disk: DiskCache) -> Self {
        Self {
            memory: MemoryCache::new(memory_capacity),
            disk,
        }
    }

    /// Recupera un valor: RAM → disco (con promoción a RAM).
    pub fn get(&self, key: &str) -> Result<Option<V>> {
        if let Some(v) = self.memory.get(&key.to_string()) {
            trace!(key, "acierto en RAM");
            return Ok(Some(v));
        }
        match self.disk.get::<V>(key)? {
            Some(v) => {
                trace!(key, "acierto en disco; se promociona a RAM");
                self.memory.put(key.to_string(), v.clone());
                Ok(Some(v))
            }
            None => Ok(None),
        }
    }

    /// Inserta el valor en ambos niveles.
    pub fn put(&self, key: &str, value: V) -> Result<()> {
        self.disk.put(key, &value)?;
        self.memory.put(key.to_string(), value);
        Ok(())
    }

    /// Fuerza el volcado del nivel de disco.
    pub fn flush(&self) -> Result<()> {
        self.disk.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escribe_en_ambos_niveles_y_promociona() {
        let dir = tempfile::tempdir().unwrap();
        let disk = DiskCache::open(dir.path()).unwrap();
        let cache: TieredCache<String> = TieredCache::new(8, disk.clone());

        cache.put("k", "v".to_string()).unwrap();
        // Está en disco directamente.
        assert_eq!(disk.get::<String>("k").unwrap(), Some("v".to_string()));

        // Una caché nueva (RAM vacía) sobre el mismo disco lo recupera y
        // lo promociona a RAM.
        let fresh: TieredCache<String> = TieredCache::new(8, disk);
        assert_eq!(fresh.get("k").unwrap(), Some("v".to_string()));
    }
}
