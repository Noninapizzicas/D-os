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
- 🧩 **Extracción estructurada** integrada en el crawler: selectores CSS y
  densidad semántica, disponibles en la librería, la CLI y la API.
- 🗃️ **Caché de dos niveles**: LRU en RAM + `sled` en disco (`TieredCache`),
  integrada en el crawler para no reprocesar páginas ya vistas.
- ⚡ **Concurrencia acotada** en `crawl_many` y crawl profundo.
- 🥷 **Modo stealth**: rotación de fingerprint, script anti-detección
  (`navigator.webdriver`, WebGL, plugins…) y comportamiento humano, aplicado
  vía CDP. Flag `--stealth`.
- 🌐 **Servidor API** (`axum`): REST + WebSocket de progreso, autenticación
  JWT y dashboard web embebido.
- 🖥️ **CLI** `crawl4rs` con los subcomandos `crawl`, `deep`, `serve`, `config`.

Además: **fetch rápido sin navegador** (`--mode fast`/`auto`), **JSON-LD /
schema.org**, **`map`** (enlaces), **`search`** (SearXNG) y **PDF digital →
Markdown**.

Por diseño, **Crawl4RS no incorpora un LLM** (ni `candle`/ONNX): eso rompería
lo ligero y duplicaría el LLM del consumidor (p. ej. Enki). Ver
[«Extracción de datos»](#extracción-de-datos). Queda pendiente la documentación
completa (mdBook).

## Instalación y uso

```bash
# Compilar
cargo build --release

# Crawl real (modo auto por defecto): intenta HTTP puro y sólo escala a
# navegador ante 403/challenge — sin abrir Chromium para páginas simples.
crawl4rs crawl https://ejemplo.com
crawl4rs crawl https://ejemplo.com --mode fast      # HTTP puro, sin navegador
crawl4rs crawl https://ejemplo.com --mode browser   # fuerza el navegador

# Sólo el "fit_markdown", filtrado por relevancia a una consulta
crawl4rs crawl https://ejemplo.com --fit --query "rust async runtime"

# Crawl profundo: sigue enlaces del mismo dominio (BFS por defecto)
crawl4rs deep https://ejemplo.com --max-depth 2 --max-pages 25
crawl4rs deep https://ejemplo.com --strategy dfs --cross-domain --json

# Con caché en disco (RAM + sled): la segunda pasada reutiliza lo ya visto
crawl4rs deep https://ejemplo.com --cache ./.crawl4rs-cache --concurrency 8

# Modo stealth: fingerprint rotado + comportamiento humano frente a WAFs
crawl4rs crawl https://ejemplo.com --stealth

# Extracción estructurada por CSS: texto, o atributos con `::attr(...)`
crawl4rs crawl https://tienda.com --json \
    --extract-css "titulo=h1" --extract-css "precio=.price" \
    --extract-css "imagen=img::attr(src)" --extract-css "enlace=a::attr(href)"

# JSON-LD / schema.org (fichas de producto sin adivinar selectores)
crawl4rs crawl https://tienda.com/producto --json --jsonld

# Mapa de enlaces de una página (ligero, sin contenido)
crawl4rs map https://ejemplo.com

# PDF digital → Markdown (detección automática por content-type/.pdf, modo fast)
crawl4rs crawl https://ejemplo.com/factura.pdf --mode fast

# A través de un proxy de salida (añade --insecure si intercepta TLS)
crawl4rs crawl https://ejemplo.com --proxy 127.0.0.1:8888 --insecure

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
`GET /crawl/{id}/result`, `GET /crawl/{id}/stream` (WebSocket), `POST /map`
(enlaces de una página), `POST /search` (búsqueda web vía SearXNG; requiere
`SEARXNG_URL`, si no degrada con 503), `GET /dashboard`, `GET /health`.

`POST /crawl` acepta además `mode` (`fast`/`browser`/`auto`), `extract_css`
(texto o `{selector, attr, many}`) y `extract_jsonld`.

### Docker

```bash
# Imagen por defecto: Debian slim + Chromium — el modo navegador funciona
# de fábrica (`crawl`, `deep`, `serve`).
docker build -t crawl4rs .
docker run -p 8080:8080 -e CRAWL4RS_JWT_SECRET=… crawl4rs

# Imagen mínima (distroless, sin navegador) para procesar HTML ya obtenido
# o servir la API sin `crawl <url>`:
docker build --target minimal -t crawl4rs:minimal .
```

### Binarios precompilados

Cada tag `vX.Y.Z` publica binarios para Linux y macOS (x86-64 y arm64) vía
GitHub Actions (`.github/workflows/release.yml`).

### Extracción de datos

Hay **dos vías**, según el tipo de esquema — y Crawl4RS **no usa LLM** en
ninguna:

1. **Determinista / repetible** (campos conocidos, alto volumen: precio,
   cantidad, sku…). Se resuelve con `extract_css` + atributos, que es barato,
   fiable y reproducible cada día:
   ```bash
   crawl4rs crawl https://tienda.com/p --json \
       --extract-css "precio=.price::attr(content)" \
       --extract-css "sku=[itemprop=sku]::attr(content)"
   ```
   Y **JSON-LD** (`--jsonld`) da la ficha `Product` estructurada sin selectores.

2. **Arbitrario / difuso** («saca lo que haya», en lenguaje natural). Se
   resuelve por **composición**: Crawl4RS devuelve `markdown` limpio +
   `jsonld`, y **el LLM del consumidor** (p. ej. el `ai-gateway` de Enki)
   aplica el esquema. Así el crawler queda ligero y desacoplado, y el LLM vive
   donde ya existe.

Un endpoint `POST /extract` sería sólo un alias ergonómico de la vía 1
(CSS/atributos); no se incluye por defecto para no duplicar superficie.

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
