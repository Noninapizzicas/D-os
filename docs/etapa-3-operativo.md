# Etapa 3 — Lo que falta para que esté TODO operativo

> Estado: **hoja de ruta** (nada implementado salvo lo marcado ✅). Cierra las
> capacidades `reservada` del `contrato-puente-v1` hasta que las **dos
> herramientas** (Crawl4RS marcha corta + Playwright marcha larga) hagan su
> trabajo **al máximo** y el motor de extracción sea operativo de verdad.
>
> Fuera de alcance (aparcado por decisión): **qué se hace con los datos**
> (agregar, comparar, persistir). Aquí solo cerramos las herramientas.

## Definición de "operativo"

El motor está operativo cuando, con los dos contenedores levantados, puede:
1. **Entrar** en un sitio con usuario/contraseña y **conservar la sesión**.
2. **Renderizar** JS e **interactuar** (scroll / "cargar más") para revelar datos.
3. **Extraer en volumen** reutilizando esa sesión, barato.
4. **Sobrevivir** a anti-bot medio (fingerprint real + proxy).
5. Todo ello **verificado en vivo** contra web pública real.

## Ya cerrado (Etapas 1–2)

- ✅ Motor de dos marchas: `FetchMode {fast|browser|auto}` con escalación.
- ✅ Puente `abrir(url)→{html}`: `PlaywrightFetcher` (Rust) + wrapper HTTP (Node).
- ✅ Cableado: `CRAWL4RS_PLAYWRIGHT_URL` → la marcha larga es Playwright.

## Etapa 3 — los `reservada → ahora`, por valor

### 1. Login → sesión ✅ (el que desbloquea el caso real)
- **Wrapper:** `POST /login { url, pasos }` ejecuta un guion (`fill`/`click`/
  `wait`) y devuelve `sesion` (`storageState`: cookies + localStorage);
  `POST /abrir { url, sesion }` abre ya autenticado.
- **Crawl4RS:** tipo [`Session`] + `PlaywrightFetcher::login/with_session`
  (marcha larga, sesión entera) y `HttpFetcher::with_session` (marcha corta,
  solo cookies — el localStorage no viaja por HTTP, honesto).
- **Verificado en vivo:** `/login` con Chromium real captura la cookie tras el
  guion; 6 tests nuevos en verde (sesión, cookie en marcha corta, sesión en el
  cuerpo, login).
- **Pendiente de la #2:** cablear la captura en la escalación automática.

### 2. Reutilización de sesión + re-login ✅
- ✅ Inyección de la `sesion` en ambas marchas (hecho en #1).
- ✅ **Lazo automático**: celda de sesión compartida (`SessionCell`) que los
  fetchers leen en caliente; trait `Authenticator` (lo implementa
  `PlaywrightFetcher::authenticator(url, pasos)`); `Crawler::with_auto_login`.
  Cuando una descarga parece **sesión perdida** (`looks_like_session_lost`: 401
  o URL de login), el crawler hace login, **refresca la celda y reintenta una
  vez** (sin bucles). Test `auto_reloguea_al_perder_sesion_y_reintenta` en verde.
- **Desbloquea:** volumen autenticado barato y sesiones que caducan sin romperse.
- ✅ **Cableado en el CLI/servidor** por variables de entorno (mismo patrón que
  `CRAWL4RS_PLAYWRIGHT_URL`):
  - `CRAWL4RS_PLAYWRIGHT_URL` = endpoint del wrapper.
  - `CRAWL4RS_LOGIN` = ruta a un JSON `{ "url": "…", "pasos": [ {tipo,selector,valor} … ] }`.
  Con ambas, `crawl`/`deep`/`serve` comparten la celda de sesión entre marchas
  y activan el lazo. Sin ellas, comportamiento idéntico (additivo). Verificado:
  el CLI lee la receta y anuncia `Auto-login: receta … vía …`.

### 3. Interacción para revelar ✅
- **Wrapper:** `POST /abrir { url, interactuar:[…] }` ejecuta un guion antes de
  leer el DOM: `fill` · `click` · `wait` · `scroll {veces,pausa_ms}` (baja al
  fondo N veces para scroll infinito / lazy-load). Compartido con `/login`.
- **Crawl4RS:** `PlaywrightFetcher::with_interact(pasos)` incluye el guion en
  cada `/abrir`. **CLI:** `CRAWL4RS_INTERACT` = ruta a un JSON de pasos.
- **Verificado en vivo:** un click revela contenido dinámico ausente sin él
  (marcador generado por JS, no en el fuente). Test en verde.
- **Desbloquea:** scroll infinito, "cargar más", pestañas, contenido tras clic.

### 4. Interceptar API interna ✅
- **Wrapper:** `POST /abrir { url, interceptar }` (`true` = todo JSON; `{contiene:
  ["/api/"]}` = filtra por URL) → captura las respuestas JSON que la página se
  pide a sí misma → `intercepted:[{url,status,json}]`. Espera `networkidle`
  para los XHR tardíos.
- **Crawl4RS:** `PlaywrightFetcher::with_intercept(cfg)`; el JSON sube por
  `FetchedPage::intercepted` → `CrawlResult::intercepted` (sale en `--json`).
  **CLI:** `CRAWL4RS_INTERCEPT` = `true`/`1` o lista de subcadenas de URL.
- **Verificado en vivo:** el `fetch('/api/precios')` de una página se captura
  limpio (`{producto, precio}`). Test en verde.
- **Desbloquea:** datos más limpios y completos que el DOM; a veces te saltas el HTML.

### 5. Endurecimiento anti-bot (freno C)
- **Wrapper:** stealth (ocultar `webdriver`, huella coherente).
- **Ambos:** capa de **proxy** (residencial para reputación de IP), ya presente
  en la marcha corta/propia — extenderla a la marcha larga.
- **Desbloquea:** Cloudflare/medios. Residual honesto: DataDome/Turnstile pueden
  ganar igual — no se promete.

### 6. Emulación
- **Wrapper:** `emular:{geo|idioma|dispositivo}`.
- **Desbloquea:** precios por región, versión móvil.

### 7. Despliegue conjunto ✅ (base)
- **Topología acordada:** el wrapper de Playwright vive **dentro del contenedor
  que ya tiene Chromium** (se conecta por CDP, `PLAYWRIGHT_CDP_URL`, sin lanzar
  un segundo navegador); **Crawl4RS va por su cuenta** en su contenedor.
- ✅ `docker-compose.yml` en la raíz: servicios `browser` (wrapper+Chromium) y
  `crawl4rs` (separado), con `CRAWL4RS_PLAYWRIGHT_URL=http://browser:8100`
  cableado y `CRAWL4RS_LOGIN` opcional documentado.
- ✅ Wrapper con doble modo: **conectar** (CDP, Chromium ya corriendo) o
  **lanzar** (imagen oficial). El contrato no cambia — es cable.
- **Desbloquea:** "levanto y funciona". Verificación en vivo del CDP contra un
  Chromium en marcha: fuera del sandbox (el sandbox no expone un CDP suelto).

### 8. Verificación en vivo (fuera del sandbox)
- Un sitio JS real (render), un **login real** (sesión), un anti-bot medio
  (escalación + stealth), un volumen pequeño (sesión reutilizada).
- **Desbloquea:** la confianza de que las cinco condiciones de "operativo" se
  cumplen de verdad, no solo en tests con fixtures.

## Enganche reservado (roza el "qué hacer con los datos")

- **Precio/dato en imagen → OCR4RS.** Cuando la extracción tope con un ráster,
  Crawl4RS delega en OCR4RS (los tres órganos se encuentran). Se apunta como
  reservado; su activación pertenece ya a la capa de datos, aparcada.

## Orden sugerido

`1 → 2` (núcleo operativo: login + volumen con sesión) · `7 → 8` (desplegar y
verificar en vivo lo mínimo) · luego `3 · 4 · 5 · 6` (ampliar cobertura y dureza
según el sitio que se resista). Cada paso es un `reservada → ahora`: rellenar el
contrato ya declarado, sin rediseñar.
