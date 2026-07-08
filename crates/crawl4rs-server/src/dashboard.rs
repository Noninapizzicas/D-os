//! Dashboard web mínimo, servido en `GET /dashboard`.

/// Página única (HTML + JS embebidos, sin dependencias externas).
pub const DASHBOARD_HTML: &str = r##"<!doctype html>
<html lang="es">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Crawl4RS · Dashboard</title>
<style>
  :root { color-scheme: light dark; }
  body { font-family: system-ui, sans-serif; max-width: 820px; margin: 2rem auto; padding: 0 1rem; }
  h1 { font-size: 1.4rem; }
  input, button { font: inherit; padding: .5rem .6rem; border-radius: .4rem; border: 1px solid #8888; }
  button { cursor: pointer; }
  .row { display: flex; gap: .5rem; flex-wrap: wrap; margin: .5rem 0; }
  #log { background: #8881; border-radius: .5rem; padding: .75rem; min-height: 6rem; white-space: pre-wrap; font-family: ui-monospace, monospace; font-size: .85rem; }
  .muted { opacity: .7; font-size: .85rem; }
</style>
</head>
<body>
  <h1>🕷️ Crawl4RS · Dashboard</h1>
  <p class="muted">Cliente mínimo de la API. El token se obtiene con la API key.</p>
  <div class="row">
    <input id="apikey" placeholder="x-api-key (vacío si abierto)" size="22">
    <button onclick="getToken()">Obtener token</button>
    <span id="tokstate" class="muted"></span>
  </div>
  <div class="row">
    <input id="url" placeholder="https://ejemplo.com" size="30">
    <input id="depth" type="number" value="1" min="0" max="5" title="profundidad" style="width:5rem">
    <button onclick="startCrawl()">Crawl</button>
  </div>
  <div id="log">Sin actividad todavía.</div>

<script>
let token = null;
const log = (m) => { const l = document.getElementById('log'); l.textContent = m + "\n" + l.textContent; };

async function getToken() {
  const key = document.getElementById('apikey').value;
  const r = await fetch('/auth/token', { method: 'POST', headers: key ? { 'x-api-key': key } : {} });
  if (!r.ok) { log('✗ token rechazado (' + r.status + ')'); return; }
  token = (await r.json()).token;
  document.getElementById('tokstate').textContent = '✓ token listo';
}

async function startCrawl() {
  if (!token) { log('Primero obtén un token.'); return; }
  const url = document.getElementById('url').value;
  const max_depth = parseInt(document.getElementById('depth').value || '1', 10);
  const r = await fetch('/crawl', {
    method: 'POST',
    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
    body: JSON.stringify({ url, max_depth })
  });
  if (!r.ok) { log('✗ crawl rechazado (' + r.status + ')'); return; }
  const { id } = await r.json();
  log('▶ trabajo ' + id + ' iniciado');
  streamJob(id);
}

function streamJob(id) {
  const proto = location.protocol === 'https:' ? 'wss' : 'ws';
  const ws = new WebSocket(proto + '://' + location.host + '/crawl/' + id + '/stream?token=' + encodeURIComponent(token));
  ws.onmessage = (e) => {
    const ev = JSON.parse(e.data);
    if (ev.event === 'page') log('  · ' + (ev.ok ? '✓' : '✗') + ' ' + ev.url + ' (' + ev.completed + ')');
    else if (ev.event === 'done') { log('■ terminado: ' + ev.completed + ' páginas'); fetchResult(id); }
    else if (ev.event === 'failed') log('■ error: ' + ev.error);
    else log('  ' + JSON.stringify(ev));
  };
  ws.onerror = () => log('WebSocket: error de conexión');
}

async function fetchResult(id) {
  const r = await fetch('/crawl/' + id + '/result', { headers: { 'Authorization': 'Bearer ' + token } });
  if (!r.ok) return;
  const data = await r.json();
  log('— resultado: ' + data.pages.length + ' páginas, ' + data.errors.length + ' errores');
}
</script>
</body>
</html>"##;
