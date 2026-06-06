'use strict';

/**
 * EventCore — el bus del sistema event-core.
 *
 * Es el unico punto por el que un modulo emite y se suscribe. Implementa el
 * contrato lenguaje/events.fiel.json (v2.0.0):
 *
 *   - Bus hibrido: EventEmitter intra-proceso + MQTT inter-core + Hooks
 *     (logging/metricas/tracing). El modulo solo ve publish / subscribe.
 *   - "Emite y se desentiende": publish() resuelve cuando el broker acepta el
 *     mensaje, NO cuando los handlers lo procesaron.
 *   - Traduccion nombre <-> topic: el modulo usa nombres con puntos
 *     (recipe.created); el bus traduce a topic con barras y prefijo de
 *     categoria (core/{core-id}/events/recipe/created).
 *   - Categorias como closed_enum, cada una con su QoS y retain.
 *   - Presencia: Last-Will + discovery por core/+/status (retain).
 *   - Respuesta sincronica via mqttRequest() con correlation_id y timeout.
 *
 * Patrones: Observer (publish/subscribe), Command (mqttRequest),
 * State (ciclo de vida de conexion), DI por constructor (broker, emitter,
 * hooks, logger y clock inyectables -> testeable sin red).
 *
 * Disciplina (de _arranque-modulo-event-core-disciplina):
 *   - Sin cache defensiva de dominio (el bus es la autoridad).
 *   - Sin catch tragador: todo error se loguea con codigo y se relanza.
 *   - Sin fallback silencioso de identidad: lo no resuelto falla ruidoso.
 */

const { randomUUID } = require('crypto');

/**
 * closed_enum de categorias: cada categoria de topic con su QoS y retain.
 * Espejo de events.fiel.json -> categorias.
 * @type {Readonly<Record<string, {qos: 0|1|2, retain: boolean}>>}
 */
const CATEGORIAS = Object.freeze({
  events:         { qos: 1, retain: false }, // hecho de dominio
  state:          { qos: 1, retain: true },  // ultimo valor importa
  commands:       { qos: 1, retain: false },
  'api/request':  { qos: 1, retain: false },
  'api/response': { qos: 1, retain: false },
  status:         { qos: 1, retain: true },  // presencia: ultimo valor importa
  heartbeat:      { qos: 0, retain: false }, // senal transitoria
  logs:           { qos: 0, retain: false }, // senal transitoria
  'ui/request':   { qos: 1, retain: false },
  'ui/response':  { qos: 1, retain: false },
});

/** Error de dominio con codigo, en linea con el contrato errors.fiel. */
class EventCoreError extends Error {
  constructor(message, code) {
    super(message);
    this.name = 'EventCoreError';
    this._code = code; // INVALID_INPUT | TIMEOUT | NOT_CONNECTED | PUBLISH_FAILED
  }
}

/** Estados del ciclo de vida de la conexion (State). */
const Estado = Object.freeze({
  DISCONNECTED: 'disconnected',
  CONNECTING: 'connecting',
  CONNECTED: 'connected',
  CLOSED: 'closed',
});

class EventCore {
  /**
   * @param {object} deps
   * @param {string} deps.coreId          Identidad de este core (obligatoria).
   * @param {object} deps.mqtt            Cliente MQTT: connect/end/publish/subscribe/on.
   * @param {object} [deps.emitter]       EventEmitter intra-proceso (DI para test).
   * @param {Array<{name?:string, fn:Function}>} [deps.hooks] Hooks transversales.
   * @param {object} [deps.logger]        { info, warn, error } (default consola).
   * @param {() => string} [deps.clock]   Reloj inyectable; devuelve ISO 8601.
   * @param {number} [deps.requestTimeoutMs] Timeout por defecto de mqttRequest (5000).
   * @param {object} [deps.presencia]     { version, modulos, apis, suscripciones } para status.
   */
  constructor(deps = {}) {
    const {
      coreId,
      mqtt,
      emitter,
      hooks = [],
      logger = console,
      clock = () => new Date().toISOString(),
      requestTimeoutMs = 5000,
      presencia = {},
    } = deps;

    if (!coreId || typeof coreId !== 'string') {
      throw new EventCoreError('coreId es obligatorio', 'INVALID_INPUT');
    }
    if (!mqtt || typeof mqtt.publish !== 'function') {
      throw new EventCoreError('se requiere un cliente mqtt valido', 'INVALID_INPUT');
    }

    this.coreId = coreId;
    this._mqtt = mqtt;
    // EventEmitter es opcional para no acoplar el require; inyectable en test.
    this._emitter = emitter || new (require('events').EventEmitter)();
    this._emitter.setMaxListeners(0);
    this._hooks = hooks;
    this._logger = logger;
    this._clock = clock;
    this._requestTimeoutMs = requestTimeoutMs;
    this._presencia = presencia;

    this._estado = Estado.DISCONNECTED;
    // pendientes NO es cache de dominio: solo correlaciona request/response vivos.
    /** @type {Map<string, {resolve:Function, reject:Function, timer:NodeJS.Timeout}>} */
    this._pendientes = new Map();
  }

  get estado() {
    return this._estado;
  }

  // ---------------------------------------------------------------------------
  // Traduccion nombre <-> topic (forma del contrato)
  // ---------------------------------------------------------------------------

  /**
   * Traduce un nombre de evento (puntos) a su topic MQTT (barras + categoria).
   * recipe.created + "events" -> core/{coreId}/events/recipe/created
   * @param {string} nombre  Nombre con puntos. Admite wildcards (+, #) en suscripcion.
   * @param {string} categoria  Una de CATEGORIAS.
   * @returns {string}
   */
  nombreATopic(nombre, categoria) {
    if (!nombre || typeof nombre !== 'string') {
      throw new EventCoreError('nombre de evento invalido', 'INVALID_INPUT');
    }
    if (!(categoria in CATEGORIAS)) {
      throw new EventCoreError(`categoria desconocida: ${categoria}`, 'INVALID_INPUT');
    }
    const detalle = nombre.replace(/\./g, '/');
    return `core/${this.coreId}/${categoria}/${detalle}`;
  }

  // ---------------------------------------------------------------------------
  // PUBLICAR — emite y se desentiende
  // ---------------------------------------------------------------------------

  /**
   * Publica un evento. Resuelve cuando el broker acepta el mensaje; NO espera a
   * los handlers. El payload se entrega tambien intra-proceso (emitter) de forma
   * inmediata. Las suscripciones asumen al-menos-una-vez (QoS 1): idempotencia.
   *
   * @param {string} nombre  Nombre con puntos (<prefix>.<entity>.<verb>).
   * @param {object} payload  Objeto JSON-serializable (obligatorio que sea objeto).
   * @param {{categoria?: string}} [opts]
   * @returns {Promise<void>}
   */
  async publish(nombre, payload, opts = {}) {
    if (payload === null || typeof payload !== 'object' || Array.isArray(payload)) {
      throw new EventCoreError('el payload debe ser un objeto', 'INVALID_INPUT');
    }
    const categoria = opts.categoria || 'events';
    if (!(categoria in CATEGORIAS)) {
      throw new EventCoreError(`categoria desconocida: ${categoria}`, 'INVALID_INPUT');
    }

    const sobre = this._enriquecer(payload);
    const { qos, retain } = CATEGORIAS[categoria];
    const topic = this.nombreATopic(nombre, categoria);

    // Intra-proceso: entrega inmediata a oyentes del mismo proceso.
    this._emitter.emit(nombre, sobre);
    // Hooks transversales (logging/metricas/tracing) — no deben tumbar el publish.
    this._correrHooks('publish', { nombre, topic, qos, retain, payload: sobre });

    try {
      await this._mqtt.publish(topic, JSON.stringify(sobre), { qos, retain });
    } catch (err) {
      // Nunca se silencia: se loguea con contexto y se relanza.
      this._logger.error('eventcore.publish.failed', {
        topic, error: err && err.message,
      });
      throw new EventCoreError(`publish fallo en ${topic}: ${err && err.message}`, 'PUBLISH_FAILED');
    }
  }

  // ---------------------------------------------------------------------------
  // SUSCRIBIR — reacciona a su ritmo, idempotente
  // ---------------------------------------------------------------------------

  /**
   * Suscribe un handler a un nombre de evento. El handler debe ser idempotente
   * o tolerar duplicados (QoS 1 = al-menos-una-vez).
   *
   * @param {string} nombre  Nombre con puntos; admite wildcards + y # por categoria/detalle.
   * @param {(payload: object) => (void|Promise<void>)} handler
   * @param {{categoria?: string}} [opts]
   * @returns {Promise<void>}
   */
  async subscribe(nombre, handler, opts = {}) {
    if (typeof handler !== 'function') {
      throw new EventCoreError('handler debe ser una funcion', 'INVALID_INPUT');
    }
    const categoria = opts.categoria || 'events';
    const topicFiltro = this.nombreATopic(nombre, categoria);

    // Envoltura: aisla el fallo del handler (no traga; loguea con codigo).
    const envuelto = async (payload) => {
      try {
        await handler(payload);
      } catch (err) {
        this._logger.error('eventcore.handler.failed', {
          nombre, error: err && err.message, code: err && err._code,
        });
        // Se loguea y se contiene: un handler caido no derriba el bus ni a los demas.
      }
    };

    this._emitter.on(nombre, envuelto);
    await this._mqtt.subscribe(topicFiltro, CATEGORIAS[categoria].qos);
  }

  // ---------------------------------------------------------------------------
  // REQUEST / RESPONSE sincronico (Command)
  // ---------------------------------------------------------------------------

  /**
   * Peticion sincronica: publica a ui/request/{domain}/{action} con un
   * correlation_id y espera la respuesta correlacionada en ui/response/{cid}.
   * Resuelve al llegar o rechaza por timeout.
   *
   * @param {string} domain
   * @param {string} action
   * @param {object} [payload]
   * @param {{timeout?: number}} [opts]
   * @returns {Promise<object>}  El payload de respuesta correlacionado.
   */
  mqttRequest(domain, action, payload = {}, opts = {}) {
    if (this._estado !== Estado.CONNECTED) {
      return Promise.reject(new EventCoreError('bus no conectado', 'NOT_CONNECTED'));
    }
    const cid = randomUUID();
    const timeoutMs = opts.timeout || this._requestTimeoutMs;

    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this._pendientes.delete(cid);
        reject(new EventCoreError(`timeout esperando ${domain}.${action}`, 'TIMEOUT'));
      }, timeoutMs);

      this._pendientes.set(cid, { resolve, reject, timer });

      this.publish(`${domain}.${action}`, { ...payload, correlation_id: cid }, {
        categoria: 'ui/request',
      }).catch((err) => {
        this._limpiarPendiente(cid);
        reject(err);
      });
    });
  }

  // ---------------------------------------------------------------------------
  // Presencia y ciclo de vida (State)
  // ---------------------------------------------------------------------------

  /**
   * Conecta al broker con Last-Will, anuncia status online (retain) y se
   * suscribe a discovery (core/+/status) y al router de respuestas.
   * @returns {Promise<void>}
   */
  async connect() {
    this._estado = Estado.CONNECTING;
    const lwt = {
      topic: `core/${this.coreId}/status`,
      payload: JSON.stringify({ state: 'offline', timestamp: this._clock() }),
      qos: 1,
      retain: true,
    };

    await this._mqtt.connect({ will: lwt });
    this._registrarRouterDeRespuestas();
    this._estado = Estado.CONNECTED;

    await this.publish('status', { state: 'online', ...this._presencia }, {
      categoria: 'status',
    });
    // Discovery: descubrir peers sin registro central.
    await this._mqtt.subscribe('core/+/status', CATEGORIAS.status.qos);
  }

  /**
   * Cierra el bus: cancela peticiones vivas, anuncia offline y desconecta.
   * @returns {Promise<void>}
   */
  async close() {
    for (const [cid, p] of this._pendientes) {
      clearTimeout(p.timer);
      p.reject(new EventCoreError('bus cerrado', 'NOT_CONNECTED'));
      this._pendientes.delete(cid);
    }
    try {
      if (this._estado === Estado.CONNECTED) {
        await this.publish('status', { state: 'offline' }, { categoria: 'status' });
      }
    } catch (err) {
      this._logger.warn('eventcore.close.status_failed', { error: err && err.message });
    }
    if (typeof this._mqtt.end === 'function') {
      await this._mqtt.end();
    }
    this._estado = Estado.CLOSED;
  }

  // ---------------------------------------------------------------------------
  // Internos
  // ---------------------------------------------------------------------------

  /** Anade campos transversales sin pisar los del dominio. */
  _enriquecer(payload) {
    return {
      timestamp: payload.timestamp || this._clock(),
      ...payload,
    };
  }

  /** Ejecuta hooks sin permitir que su fallo tumbe el flujo principal. */
  _correrHooks(fase, ctx) {
    for (const hook of this._hooks) {
      try {
        hook.fn(fase, ctx);
      } catch (err) {
        this._logger.warn('eventcore.hook.failed', {
          hook: hook.name, error: err && err.message,
        });
      }
    }
  }

  /** Enruta los mensajes MQTT entrantes: resuelve respuestas y reemite a oyentes. */
  _registrarRouterDeRespuestas() {
    this._mqtt.on('message', (topic, raw) => {
      let mensaje;
      try {
        mensaje = JSON.parse(raw.toString());
      } catch (err) {
        this._logger.error('eventcore.message.parse_failed', { topic, error: err.message });
        return;
      }

      // Respuesta correlacionada -> resuelve la promesa pendiente.
      if (topic.includes('/ui/response/') && mensaje.correlation_id) {
        const pendiente = this._pendientes.get(mensaje.correlation_id);
        if (pendiente) {
          this._limpiarPendiente(mensaje.correlation_id);
          pendiente.resolve(mensaje);
        }
        return; // respuesta tardia tras timeout: cid ya no existe, se ignora.
      }

      // Evento inter-core -> reemite intra-proceso con el nombre (puntos).
      const nombre = this._topicANombre(topic);
      if (nombre) this._emitter.emit(nombre, mensaje);
    });
  }

  /** Traduce un topic entrante de vuelta a nombre con puntos (best-effort). */
  _topicANombre(topic) {
    // core/{coreId}/{categoria...}/{detalle...} -> detalle con puntos
    const partes = topic.split('/');
    if (partes[0] !== 'core' || partes.length < 4) return null;
    // descarta core, coreId, y la(s) parte(s) de categoria conocida
    const resto = partes.slice(2).join('/');
    for (const cat of Object.keys(CATEGORIAS)) {
      const prefijo = `${cat}/`;
      if (resto.startsWith(prefijo)) {
        return resto.slice(prefijo.length).replace(/\//g, '.');
      }
    }
    return null;
  }

  _limpiarPendiente(cid) {
    const p = this._pendientes.get(cid);
    if (p) {
      clearTimeout(p.timer);
      this._pendientes.delete(cid);
    }
  }
}

module.exports = { EventCore, EventCoreError, CATEGORIAS, Estado };
