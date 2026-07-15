# Wrapper Playwright — marcha larga del puente

Wrapper HTTP fino sobre **Playwright-librería**. Es la mitad Node del puente
Crawl4RS ↔ Playwright: una costura **máquina-a-máquina y determinista**, por eso
HTTP y no MCP (el MCP de Playwright se **reserva para la capa de agente**).

Crawl4RS (Rust) lo consume con `PlaywrightFetcher` (feature `playwright`), que
hace `POST /abrir` con `reqwest`. Ver `docs/contrato-puente-prisma.md`.

## Contrato (`contrato-puente-v1`)

### Implementado

```text
POST /abrir   { "url", "sesion"?, "interactuar"?, "interceptar"?, "stealth"?, "proxy"?, "emular"? }
  200 -> { "html": "…", "final_url": "…", "status": 200, "intercepted"?: [{url,status,json}] }
  200 -> { "fallo": { "tipo": "timeout|error", "motivo": "…" } }   (no inventa HTML)
  400 -> { "fallo": { "tipo": "peticion_invalida", "motivo": "…" } }

POST /login   { "url": "https://…", "pasos": [ … ] }
  200 -> { "sesion": <storageState>, "final_url": "…" }
  200 -> { "fallo": { … } }   (no inventa sesión)

GET  /health  -> { "status": "ok", "playwright_ready": true|false }
```

- **`sesion`** en `/abrir` = `storageState` de Playwright → abre ya autenticado.
- **`interactuar`** / **`pasos`** = guion de acciones (mismo formato en `/abrir`
  y `/login`):
  `{ "tipo": "fill", "selector": "#user", "valor": "yo" }` ·
  `{ "tipo": "click", "selector": "#entrar" }` ·
  `{ "tipo": "wait", "selector"|"ms": … }` ·
  `{ "tipo": "scroll", "veces": 3, "pausa_ms": 500 }` (scroll infinito / lazy).
- **`interceptar`** en `/abrir` = `true` (todo JSON) o `{ "contiene": ["/api/"] }`
  (filtra por URL) → devuelve `intercepted:[{url,status,json}]` con lo que la
  página pide a su API interna.
- **`stealth`** (bool) = oculta señales de automatización (`navigator.webdriver`
  → `undefined`, etc.) + UA/locale realistas. Ligero; **no** promete
  DataDome/Turnstile.
- **`proxy`** = `{ "server": "http://host:puerto", "username"?, "password"? }`
  aplicado por contexto (también en `/login`).
- **`emular`** = `{ "locale"?, "timezone"?, "geo"?: {latitude,longitude},
  "movil"?: bool }` — idioma, zona horaria, geolocalización y perfil móvil.

### Declarado, RESERVADO

Las "5 líneas de más": nombradas en el contrato, para que añadirlas sea
*rellenar*, no *rediseñar*.

- **capturar** — screenshot / pdf.

## Arranque

```bash
# Local (requiere navegadores de Playwright instalados)
npm install
npm start                       # escucha en :8100

# Docker (imagen oficial de Playwright, navegadores incluidos)
docker build -t playwright-wrapper .
docker run -p 8100:8100 playwright-wrapper

# Prueba
curl -s localhost:8100/health
curl -s -X POST localhost:8100/abrir -H 'content-type: application/json' \
  -d '{"url":"https://example.com"}' | head -c 200
```

## Navegador: lanzar o conectar

Dos formas, según dónde viva el wrapper (variables de entorno):

- **`PLAYWRIGHT_CDP_URL`** → se **conecta** a un Chromium **ya corriendo** por
  CDP (p. ej. embutido en el contenedor que ya tiene Chromium). No arranca un
  segundo navegador. Ej.: `PLAYWRIGHT_CDP_URL=http://127.0.0.1:9222`.
- **sin ella** → **lanza** su propio Chromium (imagen oficial de Playwright).
  `PLAYWRIGHT_EXECUTABLE_PATH` es un escape-hatch para binarios en rutas que
  Playwright no espera.

## Topología

El wrapper vive **dentro del contenedor que ya tiene Chromium** (conectándose
por CDP); Crawl4RS va **por su cuenta**, en el suyo; hablan por HTTP en la red
de Docker. Alternativa: construir esta imagen (lanza su propio Chromium). El
contrato no cambia entre ambas — es cable. Ver `docker-compose.yml` en la raíz.

## Estado

`/abrir` (render + `sesion`), `/login` (guion → `storageState`) y `/health`
implementados; conexión por CDP o lanzamiento propio. El resto del contrato,
reservado. La verificación en vivo (web pública, login real, CDP contra un
Chromium en marcha) se hace fuera del sandbox.
