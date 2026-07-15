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

### 1. Login → sesión  · (el que desbloquea el caso real)
- **Wrapper:** `POST /abrir { url, login }` → devuelve `sesion` (`storageState`:
  cookies + localStorage). Guion de login (selectores) como primer camino.
- **Crawl4RS:** `SessionAuth` que el `Fetcher` inyecta (cookies/cabeceras) en el
  fetch HTTP **y** en la marcha larga.
- **Desbloquea:** entrar con user/contraseña; el resto del catálogo ya es visible.

### 2. Reutilización de sesión + re-login
- **Crawl4RS:** inyecta la `sesion` en el volumen (marcha corta); al detectar
  401 / redirección-a-login (`looks_like_challenge` generalizado a "sesión
  perdida") pide re-login a la marcha larga y refresca.
- **Desbloquea:** volumen autenticado barato y sesiones que caducan sin romperse.

### 3. Interacción para revelar
- **Wrapper:** `POST /abrir { url, interactuar:[scroll|click|fill_form|wait] }`
  antes de leer el DOM; cerrar banners de cookies/geo.
- **Desbloquea:** scroll infinito, "cargar más", pestañas, precio al hover.

### 4. Interceptar API interna
- **Wrapper:** `interceptar:true` → captura el JSON que la web se pide a sí
  misma → `intercepted:[...]`.
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

### 7. Despliegue conjunto
- `docker-compose.yml`: `crawl4rs` + `playwright-wrapper` en la misma red,
  `CRAWL4RS_PLAYWRIGHT_URL` cableado, healthchecks. (Embutir en uno sigue
  disponible sin tocar el contrato.)
- **Desbloquea:** "levanto y funciona".

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
