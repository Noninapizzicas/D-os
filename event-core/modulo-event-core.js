'use strict';

/**
 * ModuloEventCore — clase base ABSTRACTA de todo modulo del sistema.
 *
 * Encarna el paradigma "event-core puro": federacion en vez de centralismo
 * local. Cada modulo es soberano en su dominio PERO el bus es autoridad
 * superior a su memoria propia. La clase existe como ANDAMIO MECANICO: hace
 * facil lo correcto y lanza ante lo incorrecto, porque la conciencia
 * intelectual del paradigma no impide la regresion al escribir
 * (invariant: el_llm_escribe_contracorriente_de_su_corpus).
 *
 * Invariantes forzadas (de _arranque-modulo-event-core-disciplina):
 *   1. this.estado === null || this.estado instanceof CacheIncremental
 *   2. cada publish: payload completo antes de emitir
 *   3. cada handler: event.source.module se lee explicito (sin fallback a activo)
 *   4. toda I/O pasa por el bus; sin fs ni db directos (whitelist: filesystem,
 *      database-manager, project-manager)
 *
 * Patrones: Template Method (load/unload orquestan, onLoad/onUnload los rellena
 * la subclase), Observer (via bus), proyeccion event-sourced (CacheIncremental).
 */

const { CacheIncremental } = require('./cache-incremental');

/** Modulos a los que se permite delegar I/O por el bus. */
const IO_WHITELIST = Object.freeze(['filesystem', 'database-manager', 'project-manager']);

/**
 * Patrones de codigo prohibidos (regex) tomados literalmente del contrato.
 * auditarFuente() los usa como scaffold mecanico en CI / pre-commit.
 */
const FORBIDDEN_PATTERNS = Object.freeze([
  { id: 'cache_defensiva', regex: /this\._?cache\s*=\s*new Map/, razon: 'cache defensiva duplica la autoridad del bus' },
  { id: 'cache_por_proyecto', regex: /this\.\w+PerProject\s*=\s*new Map/, razon: 'cache por proyecto = cache defensiva' },
  { id: 'bypass_fs_require', regex: /require\(['"]fs['"]\)/, razon: 'bypass del modulo filesystem dueño del scope' },
  { id: 'bypass_fs_read', regex: /await fs\.(readdir|readFile|writeFile)/, razon: 'I/O directa salta el bus' },
  { id: 'catch_tragador', regex: /catch\s*\(\s*_?\s*\)\s*\{\s*\}/, razon: 'catch tragador oculta el error' },
  { id: 'fallback_identidad', regex: /resolveTo\w*Fallback|resolveToActive\w*/, razon: 'fallback silencioso de identidad enmascara desincronizacion' },
]);

class ModuloEventCore {
  /**
   * @param {object} deps
   * @param {import('./event-core').EventCore} deps.bus  Bus (obligatorio).
   * @param {object} deps.manifest  module.json: { name, prefix, language?, events? }.
   * @param {object} [deps.logger]  { info, warn, error }.
   */
  constructor(deps = {}) {
    if (new.target === ModuloEventCore) {
      throw Object.assign(new Error('ModuloEventCore es abstracta; extiendela'), { _code: 'INVALID_INPUT' });
    }
    // Andamio mecanico: la subclase DEBE implementar el ciclo de vida.
    if (this.onLoad === ModuloEventCore.prototype.onLoad) {
      throw Object.assign(new Error(`${new.target.name} debe implementar onLoad(context)`), { _code: 'INVALID_INPUT' });
    }
    if (this.onUnload === ModuloEventCore.prototype.onUnload) {
      throw Object.assign(new Error(`${new.target.name} debe implementar onUnload()`), { _code: 'INVALID_INPUT' });
    }

    const { bus, manifest, logger = console } = deps;
    if (!bus || typeof bus.publish !== 'function') {
      throw Object.assign(new Error('se requiere un bus event-core'), { _code: 'INVALID_INPUT' });
    }
    if (!manifest || !manifest.name || !manifest.prefix) {
      throw Object.assign(new Error('manifest requiere name y prefix'), { _code: 'INVALID_INPUT' });
    }

    this.bus = bus;
    this.manifest = manifest;
    this.name = manifest.name;
    this.prefix = manifest.prefix;
    this._logger = logger;
    // Verbos permitidos (past participle del idioma del modulo); si no se
    // declara, no se valida (responsabilidad del contrato naming).
    this._verbos = (manifest.language && manifest.language.verbs) || null;

    // Invariante 1: estado nace null; la subclase lo hace CacheIncremental en onLoad.
    this.estado = null;
  }

  // ---------------------------------------------------------------------------
  // Ciclo de vida (Template Method)
  // ---------------------------------------------------------------------------

  /**
   * Orquesta el arranque: cablea las suscripciones declaradas en el manifest
   * (lo que normalmente hace el loader) y delega en onLoad para el resto.
   * @param {object} context
   */
  async load(context) {
    const subs = (this.manifest.events && this.manifest.events.subscribes) || [];
    for (const sub of subs) {
      const handler = this[sub.handler];
      if (typeof handler !== 'function') {
        throw Object.assign(
          new Error(`handler declarado no existe: ${sub.handler}`), { _code: 'INVALID_INPUT' });
      }
      await this.bus.subscribe(sub.name, handler.bind(this), { categoria: sub.categoria || 'events' });
    }

    await this.onLoad(context); // subclase: subs extra + replay canonico del dominio

    await this.bus.publish('status', { state: 'loaded', modulo: this.name }, { categoria: 'status' });
  }

  /** Orquesta la parada. */
  async unload() {
    await this.onUnload();
    await this.bus.publish('status', { state: 'unloaded', modulo: this.name }, { categoria: 'status' });
  }

  // --- Metodos abstractos (la subclase los reemplaza) ---

  /**
   * Registrar suscriptores adicionales y reconstruir this.estado por REPLAY de
   * los eventos canonicos del propio dominio desde el bus. JAMAS leer disco.
   * @abstract @param {object} _context
   */
  async onLoad(_context) {
    throw Object.assign(new Error('onLoad no implementado'), { _code: 'INVALID_INPUT' });
  }

  /**
   * Liberar recursos sin persistir fuera del bus.
   * @abstract
   */
  async onUnload() {
    throw Object.assign(new Error('onUnload no implementado'), { _code: 'INVALID_INPUT' });
  }

  // ---------------------------------------------------------------------------
  // Helpers de dominio — hacen facil lo correcto
  // ---------------------------------------------------------------------------

  /**
   * Emite un evento de dominio. Construye el nombre <prefix>.<entity>.<verb> y
   * valida que el payload este COMPLETO antes de publicar (invariante 2).
   * @param {string} entity
   * @param {string} verb  past participle del whitelist del idioma del modulo.
   * @param {object} payload
   * @param {{required?: string[]}} [opts]  campos obligatorios del dominio.
   */
  async emit(entity, verb, payload, opts = {}) {
    if (this._verbos && !this._verbos.includes(verb)) {
      throw Object.assign(new Error(`verbo fuera de whitelist: ${verb}`), { _code: 'INVALID_INPUT' });
    }
    const requeridos = opts.required || [];
    const faltan = requeridos.filter((k) => payload == null || payload[k] === undefined);
    if (faltan.length > 0) {
      throw Object.assign(
        new Error(`payload incompleto, faltan: ${faltan.join(', ')}`), { _code: 'INVALID_INPUT' });
    }
    await this.bus.publish(`${this.prefix}.${entity}.${verb}`, payload, { categoria: 'events' });
  }

  /**
   * Lee event.source.module de forma explicita; LANZA si falta (invariante 3:
   * sin fallback a "activo").
   */
  sourceModule(event) {
    const m = event && event.source && event.source.module;
    if (!m) {
      throw Object.assign(new Error('event.source.module ausente'), { _code: 'INVALID_INPUT' });
    }
    return m;
  }

  /**
   * Crea la proyeccion de estado del modulo (CacheIncremental) — unica forma
   * permitida de mantener estado (invariante 1).
   * @param {(estado: Map, evento: object) => void} reducer
   */
  crearEstado(reducer) {
    this.estado = new CacheIncremental({ reducer });
    return this.estado;
  }

  // ---------------------------------------------------------------------------
  // I/O — SIEMPRE por el bus (invariante 4)
  // ---------------------------------------------------------------------------

  /** Lee via el modulo filesystem por el bus. Nunca fs directo. */
  async leer(path, projectId) {
    return this._ioRequest('filesystem', 'read', { path, project_id: projectId });
  }

  /** Lista via el modulo filesystem por el bus. */
  async listar(path, projectId) {
    return this._ioRequest('filesystem', 'list', { path, project_id: projectId });
  }

  /** Escribe via el modulo filesystem por el bus. */
  async escribir(path, contenido, projectId) {
    return this._ioRequest('filesystem', 'write', { path, content: contenido, project_id: projectId });
  }

  async _ioRequest(modulo, action, payload) {
    if (!IO_WHITELIST.includes(modulo)) {
      throw Object.assign(new Error(`I/O no permitida via ${modulo}`), { _code: 'INVALID_INPUT' });
    }
    return this.bus.mqttRequest(modulo, action, payload);
  }

  // ---------------------------------------------------------------------------
  // Scaffold de disciplina (estatico) — para CI / pre-commit
  // ---------------------------------------------------------------------------

  /**
   * Audita codigo fuente contra los patrones prohibidos del contrato.
   * Andamio MECANICO (no narrativo): se engancha en pre-commit/CI.
   * @param {string} codigo
   * @returns {Array<{id:string, razon:string, linea:number}>} violaciones
   */
  static auditarFuente(codigo) {
    const lineas = codigo.split('\n');
    const violaciones = [];
    lineas.forEach((linea, i) => {
      for (const p of FORBIDDEN_PATTERNS) {
        if (p.regex.test(linea)) {
          violaciones.push({ id: p.id, razon: p.razon, linea: i + 1 });
        }
      }
    });
    return violaciones;
  }
}

module.exports = { ModuloEventCore, IO_WHITELIST, FORBIDDEN_PATTERNS };
