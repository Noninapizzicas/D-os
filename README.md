# Crawl4RS

> Convierte cualquier página web en Markdown limpio y listo para LLMs — en un
> binario estático de Rust: radicalmente más rápido, ligero y fácil de
> desplegar que su contraparte en Python, con la misma filosofía **LLM-first**.

**Métrica de éxito:** extraer 1.000 páginas por minuto en una VPS de 4 GB de
RAM, con una imagen Docker < 20 MB.

---

## Estado actual

Este repositorio está en la **Fase 0 → 2** de la hoja de ruta. Lo que ya
funciona hoy, compilando y con tests en verde:

- 🧱 **Workspace** de Cargo con los 7 subcrates de la arquitectura.
- 🔤 **Pipeline de Markdown** real: `HTML → limpieza → markdown → fit_markdown`
  (encabezados, listas, citas, código, enlaces, imágenes), con eliminación de
  `script`/`style`/`nav`/`footer`.
- 🔎 **Filtros de contenido**: poda heurística por densidad de texto y
  **BM25** (Okapi, implementado desde cero) para filtrar por relevancia a una
  consulta.
- 🧩 **Estrategias de extracción**: selectores CSS y densidad semántica.
- 🗃️ **Caché LRU** en memoria.
- 🖥️ **CLI** `crawl4rs` con los subcomandos `crawl`, `deep`, `serve`, `config`.

Marcadores de posición documentados para las fases siguientes: navegador
headless (`chromiumoxide`), caché en disco (`sled`), stealth (`chaser-oxide`),
API (`axum`) y extracción con LLM local (`candle`).

## Instalación y uso

```bash
# Compilar
cargo build --release

# Procesar un HTML local → Markdown
crawl4rs crawl https://ejemplo.com --html-file pagina.html

# Sólo el "fit_markdown", filtrado por relevancia a una consulta
crawl4rs crawl https://ejemplo.com --html-file pagina.html --fit \
    --query "rust async runtime"

# Salida estructurada en JSON
crawl4rs crawl https://ejemplo.com --html-file pagina.html --json

# Ver la configuración por defecto
crawl4rs config
```

> El fetcher de red vía navegador (`crawl <url>` sin `--html-file`) llega en la
> Fase 1. Hasta entonces, se procesa HTML ya disponible con `--html-file`.

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
```

## Licencia

Distribuido bajo licencia doble **MIT OR Apache-2.0**. Consulta
[`LICENSE-MIT`](./LICENSE-MIT) y [`LICENSE-APACHE`](./LICENSE-APACHE).
