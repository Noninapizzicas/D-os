# Hoja de ruta de Crawl4RS

Seis fases, ~12-16 semanas. El estado refleja lo implementado en este
repositorio.

## Fase 0 — Fundación · ✅ hecho
- Workspace de Cargo con los subcrates.
- CI (GitHub Actions): fmt, clippy, test, build.
- README con visión y ejemplos.
- Logging estructurado con `tracing`.

## Fase 1 — Navegador y CDP · 🔜 siguiente
- Integrar `chromiumoxide` en `BrowserFetcher`.
- `BrowserPool`: lanzar, reutilizar y cerrar instancias.
- Navegación básica: `goto`, `wait_for`, `content()`.
- `SessionManager`: cookies y `localStorage`.
- Fallback a `thirtyfour` (WebDriver) si `chromiumoxide` es inestable.

## Fase 2 — Pipeline de Markdown · 🟡 en curso
- [x] `HtmlCleaner`: eliminar tags no deseados.
- [x] Conversión `HTML → Markdown`.
- [x] `PruningFilter`: densidad de texto.
- [x] `Bm25Filter`: ranking por relevancia (Okapi BM25 propio).
- [ ] Comparativa de salida contra Crawl4AI.

## Fase 3 — Extracción estructurada · 🟡 en curso
- [x] `CssSelectorStrategy`.
- [x] `SemanticDensityStrategy`.
- [ ] Integrar `readability` (artículo principal).
- [ ] `LlmExtractionStrategy` con `candle` (ONNX local).
- [ ] `deep` (crawl BFS/DFS).

## Fase 4 — Caché y optimización
- [x] `MemoryCache` (LRU en RAM).
- [ ] `DiskCache` con `sled`.
- [ ] `PredictiveCache` por hash de DOM.
- [ ] Concurrencia masiva con `tokio`; TTL adaptativo por `Cache-Control`.

## Fase 5 — Stealth y anti-detección
- [x] Catálogo de fingerprints y `StealthConfig`.
- [ ] Integrar `chaser-oxide` (CDP endurecido).
- [ ] Rotación de fingerprint y emulación de comportamiento humano.
- [ ] Proxies rotativos.

## Fase 6 — API, CLI y dashboard
- [x] CLI con `clap` (`crawl`, `deep`, `serve`, `config`).
- [x] DTOs de la API (`CrawlRequest`, `JobStatus`).
- [ ] Servidor `axum`: REST + WebSockets.
- [ ] Dashboard web y autenticación JWT.
- [ ] Imagen Docker distroless (< 20 MB) y documentación (mdBook).

## Objetivos de rendimiento

| Métrica | Crawl4AI (Python) | Crawl4RS (objetivo) |
|---------|-------------------|---------------------|
| Tiempo de arranque | ~2 s | < 100 ms |
| RAM por instancia | 200-300 MB | < 60 MB |
| Páginas/min (1 worker) | ~50 | ~200 |
| Páginas/min (10 workers) | ~300 | ~1.000+ |
| Tamaño imagen Docker | ~2 GB | < 20 MB |
| Latencia media (500 URLs) | ~3.200 ms | < 900 ms |
