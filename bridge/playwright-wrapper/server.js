'use strict';

/*
 * Wrapper HTTP fino sobre Playwright — la "marcha larga" del puente.
 *
 * Envuelve Playwright-librería y expone el contrato `contrato-puente-v1`
 * (ver docs/contrato-puente-prisma.md). Es una costura máquina-a-máquina y
 * determinista: por eso HTTP, no MCP (el MCP se reserva para la capa de
 * agente). Crawl4RS (Rust) llama a `POST /abrir` con reqwest.
 *
 * Implementado (subconjunto `ahora`):
 *   POST /abrir  { url }  ->  { html, final_url, status }   (+ { fallo } en error)
 *   GET  /health          ->  { status, playwright_ready }
 *
 * Declarado pero RESERVADO (las "5 líneas de más": se nombran, no se implementan
 * aún; añadirlas es rellenar, no rediseñar):
 *   POST /abrir  { url, interactuar?, login?, interceptar?, emular?, capturar? }
 *     -> { html, final_url, status, sesion?, intercepted?, artefactos? }
 *
 * Honestidad (verdad_obligatoria): si no puede abrir, responde { fallo } con
 * motivo; nunca inventa HTML.
 */

const http = require('http');
const { chromium } = require('playwright');

const PORT = Number(process.env.PORT) || 8100;
const NAV_TIMEOUT = Number(process.env.NAV_TIMEOUT_MS) || 30000;

// Un solo navegador para todo el proceso; un contexto nuevo por petición
// (aislamiento). Se lanza perezoso en la primera petición.
// Dos formas de tener navegador, según dónde viva el wrapper:
//  - PLAYWRIGHT_CDP_URL → se CONECTA a un Chromium YA corriendo (p. ej. embutido
//    en el contenedor que ya tiene Chromium). No arranca un segundo navegador.
//  - si no → LANZA uno propio (Docker oficial de Playwright). PLAYWRIGHT_EXECUTABLE_PATH
//    es un escape-hatch para binarios en rutas que Playwright no espera.
function lanzarOConectar() {
  if (process.env.PLAYWRIGHT_CDP_URL) {
    return chromium.connectOverCDP(process.env.PLAYWRIGHT_CDP_URL);
  }
  const opts = { headless: true };
  if (process.env.PLAYWRIGHT_EXECUTABLE_PATH) {
    opts.executablePath = process.env.PLAYWRIGHT_EXECUTABLE_PATH;
  }
  return chromium.launch(opts);
}

let browserPromise = null;
function browser() {
  if (!browserPromise) {
    browserPromise = lanzarOConectar().catch((e) => {
      browserPromise = null; // permite reintentar si falló
      throw e;
    });
  }
  return browserPromise;
}

// UA realista para el modo stealth.
const UA_STEALTH =
  'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36';

// Parche stealth LIGERO (sin plugins pesados): oculta las señales de
// automatización más comunes. No promete pasar DataDome/Turnstile — es honesto.
function parcheStealth() {
  Object.defineProperty(navigator, 'webdriver', { get: () => undefined });
  Object.defineProperty(navigator, 'languages', { get: () => ['es-ES', 'es', 'en'] });
  Object.defineProperty(navigator, 'plugins', { get: () => [1, 2, 3, 4, 5] });
  window.chrome = window.chrome || { runtime: {} };
  const q = navigator.permissions && navigator.permissions.query;
  if (q) {
    navigator.permissions.query = (p) =>
      p && p.name === 'notifications'
        ? Promise.resolve({ state: Notification.permission })
        : q(p);
  }
}

// UA móvil por defecto para la emulación de dispositivo.
const UA_MOVIL =
  'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1';

// Crea un contexto con las opciones de la petición: sesión (storageState),
// proxy, stealth (UA + parche) y emular ({ locale, timezone, geo, movil }).
// El orden importa: emular puede sobrescribir UA/locale del stealth.
async function crearContexto(b, { sesion, proxy, stealth, emular }) {
  const opts = {};
  if (sesion) opts.storageState = sesion;
  if (proxy) opts.proxy = proxy;
  if (stealth) {
    opts.userAgent = UA_STEALTH;
    opts.locale = 'es-ES';
  }
  if (emular) {
    if (emular.locale) opts.locale = emular.locale;
    if (emular.timezone) opts.timezoneId = emular.timezone;
    if (emular.geo) {
      opts.geolocation = emular.geo; // { latitude, longitude }
      opts.permissions = [...(opts.permissions || []), 'geolocation'];
    }
    if (emular.movil) {
      opts.viewport = { width: 390, height: 844 };
      opts.isMobile = true;
      opts.hasTouch = true;
      opts.deviceScaleFactor = 3;
      opts.userAgent = emular.ua_movil || UA_MOVIL;
    }
  }
  const context = await b.newContext(opts);
  if (stealth) await context.addInitScript(parcheStealth);
  return context;
}

// Ejecuta un guion de pasos sobre la página. Compartido por login e interacción.
//   fill   { selector, valor }
//   click  { selector }
//   wait   { selector | ms }
//   scroll { veces?, pausa_ms? }  → baja al fondo N veces (scroll infinito / lazy)
async function ejecutarPasos(page, pasos) {
  for (const paso of pasos || []) {
    switch (paso.tipo) {
      case 'fill':
        await page.fill(paso.selector, String(paso.valor ?? ''));
        break;
      case 'click':
        await page.click(paso.selector);
        break;
      case 'wait':
        if (paso.selector) await page.waitForSelector(paso.selector, { timeout: NAV_TIMEOUT });
        else await page.waitForTimeout(Number(paso.ms) || 500);
        break;
      case 'scroll': {
        const veces = Number(paso.veces) || 1;
        const pausa = Number(paso.pausa_ms) || 500;
        for (let i = 0; i < veces; i += 1) {
          await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
          await page.waitForTimeout(pausa);
        }
        break;
      }
      default:
        throw new Error(`paso desconocido: ${paso.tipo}`);
    }
  }
}

// Abre una página. `sesion` (storageState) → arranca autenticado. `interactuar`
// → guion de pasos antes de leer el DOM. `interceptar` → captura el JSON que la
// página pide a su API interna (`true` = todo JSON; `{contiene:[…]}` = filtra por
// URL). RESERVADO: emular, capturar.
async function abrir({ url, sesion, interactuar, interceptar, stealth, proxy, emular }) {
  const b = await browser();
  const context = await crearContexto(b, { sesion, proxy, stealth, emular });
  try {
    const page = await context.newPage();

    const capturas = [];
    if (interceptar) {
      const filtros = Array.isArray(interceptar.contiene) ? interceptar.contiene : null;
      page.on('response', (resp) => {
        const ct = resp.headers()['content-type'] || '';
        if (!ct.includes('json')) return;
        const u = resp.url();
        if (filtros && !filtros.some((f) => u.includes(f))) return;
        // Se aplaza el cuerpo: se resuelve al final (no bloquea la navegación).
        capturas.push(
          resp
            .json()
            .then((j) => ({ url: u, status: resp.status(), json: j }))
            .catch(() => null),
        );
      });
    }

    const resp = await page.goto(url, {
      waitUntil: 'domcontentloaded',
      timeout: NAV_TIMEOUT,
    });
    await ejecutarPasos(page, interactuar);
    if (interceptar) {
      // Deja aterrizar los XHR tardíos antes de recoger.
      await page.waitForLoadState('networkidle', { timeout: 5000 }).catch(() => {});
    }
    const html = await page.content();
    const intercepted = interceptar ? (await Promise.all(capturas)).filter(Boolean) : [];
    return {
      html,
      final_url: page.url(),
      status: resp ? resp.status() : null,
      intercepted,
    };
  } finally {
    await context.close();
  }
}

// Ejecuta un guion de pasos (fill/click/wait) y captura la sesión resultante
// (storageState = cookies + localStorage). No inventa nada: si un paso falla,
// el llamador recibe el fallo.
async function login({ url, pasos, stealth, proxy, emular }) {
  const b = await browser();
  const context = await crearContexto(b, { proxy, stealth, emular });
  try {
    const page = await context.newPage();
    await page.goto(url, { waitUntil: 'domcontentloaded', timeout: NAV_TIMEOUT });
    await ejecutarPasos(page, pasos);
    const sesion = await context.storageState();
    return { sesion, final_url: page.url() };
  } finally {
    await context.close();
  }
}

function enviar(res, code, obj) {
  const s = JSON.stringify(obj);
  res.writeHead(code, {
    'Content-Type': 'application/json',
    'Content-Length': Buffer.byteLength(s),
  });
  res.end(s);
}

const server = http.createServer((req, res) => {
  if (req.method === 'GET' && req.url === '/health') {
    return enviar(res, 200, { status: 'ok', playwright_ready: browserPromise !== null });
  }

  if (req.method === 'POST' && (req.url === '/abrir' || req.url === '/login')) {
    const ruta = req.url;
    let body = '';
    req.on('data', (c) => {
      body += c;
      if (body.length > 5e7) req.destroy(); // guarda contra cuerpos gigantes
    });
    req.on('end', async () => {
      let datos;
      try {
        datos = JSON.parse(body || '{}');
      } catch {
        return enviar(res, 400, { fallo: { tipo: 'peticion_invalida', motivo: 'JSON ilegible' } });
      }
      if (!datos.url) {
        return enviar(res, 400, { fallo: { tipo: 'peticion_invalida', motivo: 'falta url' } });
      }
      try {
        const out = ruta === '/login' ? await login(datos) : await abrir(datos);
        enviar(res, 200, out);
      } catch (e) {
        // No inventamos nada: reportamos el fallo real.
        const motivo = (e && e.message) ? String(e.message) : String(e);
        const tipo = /timeout/i.test(motivo) ? 'timeout' : 'error';
        enviar(res, 200, { fallo: { tipo, motivo } });
      }
    });
    return;
  }

  enviar(res, 404, { fallo: { tipo: 'no_encontrado', motivo: req.url } });
});

server.listen(PORT, () => {
  // eslint-disable-next-line no-console
  console.error(`wrapper Playwright escuchando en :${PORT}`);
});
