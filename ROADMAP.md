# Hoja de ruta de Crawl4RS

Seis fases, ~12-16 semanas. El estado refleja lo implementado en este
repositorio.

## Fase 0 — Fundación · ✅ hecho
- Workspace de Cargo con los subcrates.
- CI (GitHub Actions): fmt, clippy, test, build.
- README con visión y ejemplos.
- Logging estructurado con `tracing`.

## Fase 1 — Navegador y CDP · ✅ hecho
- [x] Integrar `chromiumoxide` en `BrowserFetcher` (feature `browser`, activa
      por defecto; desactivable para builds ligeros).
- [x] `BrowserPool`: una instancia de Chromium reutilizada, límite de pestañas
      concurrentes por semáforo, perfil temporal único por pool y cierre
      ordenado.
- [x] Navegación básica: `goto`, espera de carga (`wait_for_navigation`),
      `content()`, timeout configurable.
- [x] `SessionManager`: persistencia de cookies y `localStorage` por perfil en
      disco (JSON), verificada entre dos navegadores distintos.
- [x] Tests de integración con Chromium real (`cargo test -- --ignored`);
      detección del ejecutable vía `$CRAWL4RS_CHROME` o rutas conocidas.
- [ ] Fallback a `thirtyfour` (WebDriver) si `chromiumoxide` es inestable.
- [ ] Captura del código de estado HTTP vía eventos de red.

## Fase 2 — Pipeline de Markdown · 🟡 en curso
- [x] `HtmlCleaner`: eliminar tags no deseados.
- [x] Conversión `HTML → Markdown`.
- [x] `PruningFilter`: densidad de texto.
- [x] `Bm25Filter`: ranking por relevancia (Okapi BM25 propio).
- [ ] Comparativa de salida contra Crawl4AI.

## Fase 3 — Extracción estructurada y crawl profundo · 🟡 en curso
- [x] `CssSelectorStrategy`.
- [x] `SemanticDensityStrategy`.
- [x] `deep` (crawl BFS/DFS): recorrido con `Crawler::crawl_deep`, respeta
      `max_pages`/`max_depth`/estrategia/`same_domain`, resuelve enlaces
      relativos, deduplica y aísla el dominio. Subcomando `crawl4rs deep`.
- [ ] Integrar `readability` (artículo principal).
- [ ] `LlmExtractionStrategy` con `candle` (ONNX local).

## Fase 4 — Caché y optimización · 🟡 en curso
- [x] `MemoryCache` (LRU en RAM).
- [x] `DiskCache` con `sled` (valores JSON, persistente entre ejecuciones).
- [x] `TieredCache`: RAM por delante de disco, con promoción.
- [x] `ResultCache` integrada en `Crawler` (feature `cache`): las páginas ya
      vistas no se descargan ni reprocesan. Medido: crawl profundo de 4
      páginas ~0,73 s en frío → ~0,06 s en caliente (>10×; ni siquiera se
      lanza Chromium).
- [x] Concurrencia acotada: `crawl_many` y `crawl_deep` descargan en paralelo
      (`config.concurrency`), recorrido por oleadas.
- [x] `dom_signature` (en `crawl4rs-markdown`): firma estructural del DOM,
      base de la caché predictiva por plantilla.
- [ ] `PredictiveCache`: reutilizar lógica de extracción entre URLs con la
      misma firma de DOM.
- [ ] TTL adaptativo por cabeceras `Cache-Control`.

## Fase 5 — Stealth y anti-detección · 🟡 en curso
- [x] Catálogo de fingerprints y `StealthConfig`.
- [x] `StealthEngine`: rotación round-robin de fingerprint, script de
      ocultación (neutraliza `navigator.webdriver`, `languages`, `platform`,
      `plugins`, vendor de WebGL, `window.chrome`) y retardos "humanos"
      deterministas (xorshift).
- [x] Aplicación vía CDP en `BrowserPool`: UA/idioma/viewport por página,
      `addScriptToEvaluateOnNewDocument` antes de navegar, movimiento de ratón
      y pausas. Flag `--stealth` en la CLI.
- [ ] CDP endurecido a la manera de `chaser-oxide` (patch de más señales CDP).
- [ ] Proxies rotativos.

## Fase 6 — API, CLI y dashboard · 🟡 en curso
- [x] CLI con `clap` (`crawl`, `deep`, `serve`, `config`).
- [x] Servidor `axum`: `POST /crawl`, `GET /crawl/{id}/status`,
      `GET /crawl/{id}/result`, `GET /health`.
- [x] Streaming de progreso por WebSocket (`GET /crawl/{id}/stream`),
      alimentado por `crawl_deep_with`.
- [x] Autenticación JWT (HS256, `rust_crypto`): `POST /auth/token` con
      `x-api-key`, middleware `Bearer` en las rutas protegidas.
- [x] Dashboard web mínimo embebido (`GET /dashboard`).
- [x] `Dockerfile` multi-etapa con imagen `distroless/cc` (binario ~6 MB;
      el navegador se aporta aparte).
- [ ] Documentación completa (mdBook).

## Objetivos de rendimiento

| Métrica | Crawl4AI (Python) | Crawl4RS (objetivo) |
|---------|-------------------|---------------------|
| Tiempo de arranque | ~2 s | < 100 ms |
| RAM por instancia | 200-300 MB | < 60 MB |
| Páginas/min (1 worker) | ~50 | ~200 |
| Páginas/min (10 workers) | ~300 | ~1.000+ |
| Tamaño imagen Docker | ~2 GB | < 20 MB |
| Latencia media (500 URLs) | ~3.200 ms | < 900 ms |
