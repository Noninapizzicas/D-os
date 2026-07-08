# Hoja de ruta de Crawl4RS

Siete fases, ~12-16 semanas. El estado refleja lo implementado en este
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
- [x] Extracción **integrada en el crawler** (`Crawler::with_extraction`):
      puebla `CrawlResult::extracted`. Expuesta en la CLI (`--extract-css
      nombre=sel`, `--extract-semantic`) y en la API (`extract_css`,
      `extract_semantic` en `POST /crawl`; `extracted` por página).
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
- [x] Soporte de **proxy** de salida (`--proxy` / `BrowserPoolConfig.proxy`,
      pasado como `--proxy-server`). La rotación por petición requiere
      contextos de navegador separados (pendiente).
- [ ] CDP endurecido a la manera de `chaser-oxide` (patch de más señales CDP).

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
- [ ] Endurecer el secreto JWT (safe-by-default): sin `CRAWL4RS_JWT_SECRET`,
      generar uno ALEATORIO en el arranque y avisarlo por log — nunca el default
      conocido `crawl4rs-dev-secret-cambia-esto` (hoy forjable por cualquiera).
      Además, NEGARSE a arrancar (fail-closed) si el servidor bindea a interfaz
      pública (0.0.0.0) sin un secreto explícito. No bloquea el desarrollo local.
- [ ] Documentación completa (mdBook).

## Fase 7 — Transporte MQTT nativo (órgano de bus) · 🔜 siguiente

Un crawler que solo habla HTTP es una isla en un sistema event-driven puro
(Enki: MQTT sobre WebSocket). El `Crawler` de `crawl4rs-core` es
transport-agnóstico —ya lo envuelven la CLI y el servidor `axum`—, así que un
tercer front-end que hable MQTT lo vuelve un órgano de primera clase en el bus,
sin puente traductor y con `correlation_id` fluyendo natural. Un motor, N caras.

- [ ] Nuevo crate `crawl4rs-mqtt` (feature `mqtt`, desactivable): cliente
      `rumqttc` sobre TCP y WebSocket (`wss://`), que envuelve el MISMO `Crawler`.
- [ ] Suscripción a `core/<core_id>/api/request/crawl/<accion>`
      (`crawl` · `deep` · `fit`), QoS 1.
- [ ] Parseo del request `{correlation_id, request_id, url, query?, stealth?,
      deep?{max_pages, max_depth, same_domain}}`; ante payload malformado, emite
      el par `*.failed` en vez de caer.
- [ ] Ejecución sobre `Crawler` (`crawl` / `crawl_deep`), reutilizando caché,
      stealth y navegador (las mismas features que HTTP y CLI).
- [ ] Respuesta a `core/<core_id>/api/response/<correlation_id>`
      `{status, markdown, fit_markdown, extraido, paginas?}`, QoS 1.
- [ ] Par canónico `*.failed`: emisión a `core/<core_id>/events/crawl/failed`
      ante cualquier error (cada flujo cierra su círculo).
- [ ] Idempotencia por `correlation_id`: dedup de reintentos (LRU acotada) para
      no re-crawlear ante re-entregas de QoS 1.
- [ ] Configuración genérica por entorno: `CRAWL4RS_MQTT_BROKER`,
      `CRAWL4RS_CORE_ID`, prefijo de tópicos y credenciales del broker; sin ellos
      el modo MQTT no arranca (fail-closed). Crawl4RS sigue siendo standalone:
      Enki solo inyecta SUS convenciones.
- [ ] Reconexión con backoff exponencial y re-suscripción (resiliencia de
      broker); métricas de conexión, latencia y tasa de error.
- [ ] Subcomando `crawl4rs mqtt` en la CLI (junto a `serve`).
- [ ] Tests: `request → response` y el par `*.failed` contra un broker embebido
      o mock; verificación del dedup por `correlation_id`.
- [ ] `Dockerfile`: modo `mqtt` (binario + Chromium) que conecta al broker y
      aparece como un órgano más — sin HTTP en el camino de Enki.

Nota: el HTTP (Fase 6) NO se retira. HTTP para uso standalone/genérico; MQTT
para el bus de Enki. El contenedor "todo incluido" (Rust + Chromium) corre en
modo `mqtt`, gateado on-demand y con `--memory` en hosts pequeños (un Chromium
residente pesa; el binario ~6 MB no).

## Objetivos de rendimiento

| Métrica | Crawl4AI (Python) | Crawl4RS (objetivo) |
|---------|-------------------|---------------------|
| Tiempo de arranque | ~2 s | < 100 ms |
| RAM por instancia | 200-300 MB | < 60 MB |
| Páginas/min (1 worker) | ~50 | ~200 |
| Páginas/min (10 workers) | ~300 | ~1.000+ |
| Tamaño imagen Docker | ~2 GB | < 20 MB |
| Latencia media (500 URLs) | ~3.200 ms | < 900 ms |
