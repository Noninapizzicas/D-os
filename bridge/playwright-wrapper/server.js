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
let browserPromise = null;
function browser() {
  if (!browserPromise) {
    // Escape-hatch para entornos donde el binario no está en la ruta que
    // Playwright espera (revisiones que no cuadran). En el Docker oficial no
    // hace falta. Ver PLAYWRIGHT_BROWSERS_PATH/executablePath.
    const opts = { headless: true };
    if (process.env.PLAYWRIGHT_EXECUTABLE_PATH) {
      opts.executablePath = process.env.PLAYWRIGHT_EXECUTABLE_PATH;
    }
    browserPromise = chromium.launch(opts).catch((e) => {
      browserPromise = null; // permite reintentar si el lanzamiento falló
      throw e;
    });
  }
  return browserPromise;
}

async function abrir({ url }) {
  const b = await browser();
  const context = await b.newContext();
  try {
    const page = await context.newPage();
    const resp = await page.goto(url, {
      waitUntil: 'domcontentloaded',
      timeout: NAV_TIMEOUT,
    });
    const html = await page.content();
    return {
      html,
      final_url: page.url(),
      status: resp ? resp.status() : null,
    };
    // RESERVADO: interactuar (scroll/click), login -> sesion (storageState),
    // interceptar (page.on('response')), emular (context locale/geo),
    // capturar (screenshot/pdf). Se añaden aquí sin tocar el contrato.
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

  if (req.method === 'POST' && req.url === '/abrir') {
    let body = '';
    req.on('data', (c) => {
      body += c;
      if (body.length > 1e6) req.destroy(); // guarda contra cuerpos gigantes
    });
    req.on('end', async () => {
      let url;
      try {
        ({ url } = JSON.parse(body || '{}'));
      } catch {
        return enviar(res, 400, { fallo: { tipo: 'peticion_invalida', motivo: 'JSON ilegible' } });
      }
      if (!url) {
        return enviar(res, 400, { fallo: { tipo: 'peticion_invalida', motivo: 'falta url' } });
      }
      try {
        const out = await abrir({ url });
        enviar(res, 200, out);
      } catch (e) {
        // No inventamos HTML: reportamos el fallo real.
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
