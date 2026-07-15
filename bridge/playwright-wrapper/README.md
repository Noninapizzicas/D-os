# Wrapper Playwright — marcha larga del puente

Wrapper HTTP fino sobre **Playwright-librería**. Es la mitad Node del puente
Crawl4RS ↔ Playwright: una costura **máquina-a-máquina y determinista**, por eso
HTTP y no MCP (el MCP de Playwright se **reserva para la capa de agente**).

Crawl4RS (Rust) lo consume con `PlaywrightFetcher` (feature `playwright`), que
hace `POST /abrir` con `reqwest`. Ver `docs/contrato-puente-prisma.md`.

## Contrato (`contrato-puente-v1`)

### Implementado

```text
POST /abrir   { "url": "https://…", "sesion"? }
  200 -> { "html": "<!doctype…>", "final_url": "https://…", "status": 200 }
  200 -> { "fallo": { "tipo": "timeout|error", "motivo": "…" } }   (no inventa HTML)
  400 -> { "fallo": { "tipo": "peticion_invalida", "motivo": "…" } }

POST /login   { "url": "https://…", "pasos": [ … ] }
  200 -> { "sesion": <storageState>, "final_url": "…" }
  200 -> { "fallo": { … } }   (no inventa sesión)

GET  /health  -> { "status": "ok", "playwright_ready": true|false }
```

- **`sesion`** en `/abrir` = `storageState` de Playwright → abre ya autenticado.
- **`pasos`** en `/login` = guion de acciones que captura la sesión:
  `{ "tipo": "fill", "selector": "#user", "valor": "yo" }` ·
  `{ "tipo": "click", "selector": "#entrar" }` ·
  `{ "tipo": "wait", "selector"|"ms": … }`.

### Declarado, RESERVADO

Las "5 líneas de más": nombradas en el contrato, para que añadirlas sea
*rellenar*, no *rediseñar*.

- **interactuar** en `/abrir` — scroll / click / fill_form antes de leer el DOM.
- **interceptar** — capturar el JSON de la API interna del sitio.
- **emular** — dispositivo / geo / idioma.
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

## Topología

El wrapper vive **dentro del contenedor de Playwright**; Crawl4RS en el suyo;
hablan por HTTP en la red de Docker. Embutir todo en un contenedor sigue
disponible sin tocar el contrato — es cable, no contrato.

## Estado

`/abrir` (render simple) y `/health` implementados. El resto del contrato,
reservado. La verificación en vivo contra la web pública se hace fuera del
sandbox (egress y navegador gestionados por el entorno).
