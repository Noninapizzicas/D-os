'use strict';

/**
 * CacheIncremental — proyeccion en memoria reconstruida por REPLAY de eventos
 * canonicos del dominio propio.
 *
 * NO es una cache defensiva. La diferencia con `new Map()` (patron prohibido,
 * tendencia T1 del contrato) es la *autoridad*:
 *   - una cache defensiva memoriza para evitar ir al bus -> duplica autoridad.
 *   - CacheIncremental se alimenta SOLO de eventos canonicos del bus y se
 *     reconstruye en onLoad -> el bus sigue siendo la unica fuente de verdad.
 *
 * `get(id)` falla ruidoso ante un id no resuelto: no busca "algo parecido"
 * (tendencia T2, fallback silencioso de identidad).
 */
class CacheIncremental {
  /**
   * @param {object} opts
   * @param {(estado: Map, evento: object) => void} opts.reducer
   *   Aplica un evento canonico sobre el estado. Lo define el dominio.
   */
  constructor({ reducer }) {
    if (typeof reducer !== 'function') {
      throw Object.assign(new Error('CacheIncremental requiere un reducer'), { _code: 'INVALID_INPUT' });
    }
    this._reducer = reducer;
    /** @type {Map<string, object>} proyeccion; clave = id de entidad. */
    this._proyeccion = new Map();
  }

  /** Aplica un evento canonico a la proyeccion (replay o llegada en vivo). */
  apply(evento) {
    this._reducer(this._proyeccion, evento);
  }

  /** Reconstruye desde cero replayando una secuencia ordenada de eventos. */
  rebuild(eventos) {
    this._proyeccion.clear();
    for (const e of eventos) this._reducer(this._proyeccion, e);
  }

  has(id) {
    return this._proyeccion.has(id);
  }

  /** Devuelve la entidad o LANZA si el id no esta resuelto (sin fallback). */
  get(id) {
    if (!this._proyeccion.has(id)) {
      throw Object.assign(new Error(`id no resuelto: ${id}`), { _code: 'INVALID_INPUT' });
    }
    return this._proyeccion.get(id);
  }

  /** Vista de solo lectura (copia superficial) para iterar sin mutar. */
  snapshot() {
    return new Map(this._proyeccion);
  }

  get size() {
    return this._proyeccion.size;
  }
}

module.exports = { CacheIncremental };
