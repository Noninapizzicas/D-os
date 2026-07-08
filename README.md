# Crawl4RS

> Convierte cualquier página web en Markdown limpio y listo para LLMs — en un
> binario estático de Rust: radicalmente más rápido, ligero y fácil de
> desplegar que su contraparte en Python, con la misma filosofía **LLM-first**.

**Métrica de éxito:** extraer 1.000 páginas por minuto en una VPS de 4 GB de
RAM, con una imagen Docker < 20 MB.

---

## Estado actual

Este repositorio está en la **Fase 1 → 3** de la hoja de ruta. Lo que ya
funciona hoy, compilando y con tests en verde:

- 🧱 **Workspace** de Cargo con los 7 subcrates de la arquitectura.
- 🌐 **Navegador real vía CDP** (`chromiumoxide`): `BrowserPool` con límite de
  pestañas concurrentes, `BrowserFetcher` de arranque perezoso y
  `SessionManager` que persiste cookies y `localStorage` por perfil.
- 🔤 **Pipeline de Markdown** real: `HTML → limpieza → markdown → fit_markdown`
  (encabezados, listas, citas, código, enlaces, imágenes), con eliminación de
  `script`/`style`/`nav`/`footer`.
- 🔎 **Filtros de contenido**: poda heurística por densidad de texto y
  **BM25** (Okapi, implementado desde cero) para filtrar por relevancia a una
  consulta.
- 🧩 **Estrategias de extracción**: selectores CSS y densidad semántica.
- 🗃️ **Caché de dos niveles**: LRU en RAM + `sled` en disco (`TieredCache`),
  integrada en el crawler para no reprocesar páginas ya vistas.
- ⚡ **Concurrencia acotada** en `crawl_many` y crawl profundo.
- 🥷 **Modo stealth**: rotación de fingerprint, script anti-detección
  (`navigator.webdriver`, WebGL, plugins…) y comportamiento humano, aplicado
  vía CDP. Flag `--stealth`.
- 🌐 **Servidor API** (`axum`): REST + WebSocket de progreso, autenticación
  JWT y dashboard web embebido.
- 🖥️ **CLI** `crawl4rs` con los subcomandos `crawl`, `deep`, `serve`, `config`.

Queda pendiente la extracción con LLM local (`candle`) y la documentación
completa (mdBook).

## Instalación y uso

```bash
# Compilar
cargo build --release

# Crawl real: lanza Chromium headless, navega y convierte a Markdown
crawl4rs crawl https://ejemplo.com

# Sólo el "fit_markdown", filtrado por relevancia a una consulta
crawl4rs crawl https://ejemplo.com --fit --query "rust async runtime"

# Crawl profundo: sigue enlaces del mismo dominio (BFS por defecto)
crawl4rs deep https://ejemplo.com --max-depth 2 --max-pages 25
crawl4rs deep https://ejemplo.com --strategy dfs --cross-domain --json

# Con caché en disco (RAM + sled): la segunda pasada reutiliza lo ya visto
crawl4rs deep https://ejemplo.com --cache ./.crawl4rs-cache --concurrency 8

# Modo stealth: fingerprint rotado + comportamiento humano frente a WAFs
crawl4rs crawl https://ejemplo.com --stealth

# Procesar un HTML local sin lanzar navegador
crawl4rs crawl https://ejemplo.com --html-file pagina.html

# Salida estructurada en JSON
crawl4rs crawl https://ejemplo.com --json

# Ver la configuración por defecto
crawl4rs config
```

> Se necesita un Chromium/Chrome instalado. Se busca automáticamente en rutas
> conocidas; puede fijarse con `CRAWL4RS_CHROME=/ruta/al/chromium`. Los builds
> sin navegador son posibles con `--no-default-features` en `crawl4rs-core`.

### Servidor API

```bash
# Lanza el servidor (dashboard en /dashboard). Restringe la emisión de
# tokens con CRAWL4RS_API_KEY y firma los JWT con CRAWL4RS_JWT_SECRET.
CRAWL4RS_API_KEY=mi-clave crawl4rs serve --port 8080

# 1) Obtener un token
TOKEN=$(curl -s -XPOST localhost:8080/auth/token -H "x-api-key: mi-clave" | jq -r .token)
# 2) Lanzar un crawl
ID=$(curl -s -XPOST localhost:8080/crawl -H "Authorization: Bearer $TOKEN" \
     -H 'Content-Type: application/json' \
     -d '{"url":"https://ejemplo.com","max_depth":1}' | jq -r .id)
# 3) Consultar el resultado (o suscribirse a WS /crawl/$ID/stream?token=…)
curl -s localhost:8080/crawl/$ID/result -H "Authorization: Bearer $TOKEN"
```

Endpoints: `POST /auth/token`, `POST /crawl`, `GET /crawl/{id}/status`,
`GET /crawl/{id}/result`, `GET /crawl/{id}/stream` (WebSocket), `GET /dashboard`,
`GET /health`.

### Docker

```bash
docker build -t crawl4rs .
docker run -p 8080:8080 -e CRAWL4RS_JWT_SECRET=… crawl4rs
```

La imagen (`distroless/cc`) empaqueta sólo el binario; para el modo navegador
aporta un Chromium accesible y apunta `CRAWL4RS_CHROME` a él.

### Como biblioteca

```rust
use std::sync::Arc;
use crawl4rs_core::{Crawler, CrawlConfig, StaticFetcher};

# async fn demo() -> crawl4rs_core::Result<()> {
let html = "<article><h1>Hola</h1><p>Mundo</p></article>";
let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
let result = crawler.crawl("https://ejemplo.test", &CrawlConfig::default()).await?;
println!("{}", result.markdown);
# Ok(())
# }
```

## Arquitectura

```text
┌─────────────────────────────────────────────────────────────────┐
│                         CLI / API Server                        │
├─────────────────────────────────────────────────────────────────┤
│                      Orchestrator (Crawl Engine)                │
├───────────────┬───────────────────┬────────────────────────────┤
│  Browser Pool │  Content Pipeline │  Extraction Strategies      │
│  (chromium-   │  (HTML → MD →     │  (BM25, Pruning, LLM,       │
│   oxide)      │   Fit Markdown)   │   CSS/XPath, Semantic)      │
├───────────────┴───────────────────┴────────────────────────────┤
│                    Cache Layer (sled / LRU)                     │
├─────────────────────────────────────────────────────────────────┤
│                    Async Runtime (tokio)                        │
└─────────────────────────────────────────────────────────────────┘
```

### Crates del workspace

| Crate | Responsabilidad |
|-------|-----------------|
| `crawl4rs-core` | Orquestación del crawl: `Fetcher` → pipeline → `CrawlResult`. |
| `crawl4rs-markdown` | `HTML → markdown → fit_markdown`; filtros de poda y BM25. |
| `crawl4rs-extract` | Extracción estructurada: CSS, densidad semántica, (LLM). |
| `crawl4rs-cache` | Caché LRU en RAM; disco (`sled`) y predictiva en la Fase 4. |
| `crawl4rs-stealth` | Anti-detección: fingerprints y comportamiento (Fase 5). |
| `crawl4rs-server` | Contrato de la API HTTP/WebSocket (`axum`, Fase 6). |
| `crawl4rs-cli` | Binario `crawl4rs`. |

Consulta la hoja de ruta completa en [`ROADMAP.md`](./ROADMAP.md).

## Desarrollo

```bash
cargo test          # tests unitarios y doctests
cargo clippy --all-targets
cargo fmt --all --check

# Tests de integración con un Chromium real (no corren en CI)
cargo test -p crawl4rs-core -- --ignored
```

## Licencia

Distribuido bajo licencia doble **MIT OR Apache-2.0**. Consulta
[`LICENSE-MIT`](./LICENSE-MIT) y [`LICENSE-APACHE`](./LICENSE-APACHE).
