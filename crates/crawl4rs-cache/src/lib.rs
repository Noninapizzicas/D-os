//! # crawl4rs-cache
//!
//! Capa de caché para evitar reprocesar páginas.
//!
//! - [`MemoryCache`]: LRU en RAM, acceso ultrarrápido (disponible).
//! - [`Cache`]: trait común para futuros backends (disco con `sled`,
//!   caché predictiva por hash de DOM) — Fase 4 de la hoja de ruta.

use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use lru::LruCache;

/// Interfaz común de caché clave→valor.
pub trait Cache<K, V> {
    /// Recupera un valor clonado, si existe.
    fn get(&self, key: &K) -> Option<V>;
    /// Inserta o actualiza un valor.
    fn put(&self, key: K, value: V);
    /// Número de entradas vivas.
    fn len(&self) -> usize;
    /// Indica si la caché está vacía.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Caché LRU en memoria, segura entre hilos.
pub struct MemoryCache<K: Hash + Eq, V: Clone> {
    inner: Mutex<LruCache<K, V>>,
}

impl<K: Hash + Eq, V: Clone> MemoryCache<K, V> {
    /// Crea una caché con capacidad máxima `capacity` (≥ 1).
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self {
            inner: Mutex::new(LruCache::new(cap)),
        }
    }
}

impl<K: Hash + Eq, V: Clone> Cache<K, V> for MemoryCache<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        let mut guard = self.inner.lock().unwrap();
        guard.get(key).cloned()
    }

    fn put(&self, key: K, value: V) {
        let mut guard = self.inner.lock().unwrap();
        guard.put(key, value);
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lru_expulsa_la_entrada_mas_antigua() {
        let cache: MemoryCache<String, u32> = MemoryCache::new(2);
        cache.put("a".into(), 1);
        cache.put("b".into(), 2);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        // Al insertar "c", "b" es la menos usada recientemente y se expulsa.
        cache.put("c".into(), 3);
        assert_eq!(cache.get(&"b".to_string()), None);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"c".to_string()), Some(3));
        assert_eq!(cache.len(), 2);
    }
}
