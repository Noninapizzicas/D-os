# Etapa 1 — Motor web de dos marchas (Crawl4RS + Playwright)

> Estado: **diseño acordado**, sin implementar. Este documento *apunta* la
> primera etapa y profundiza en los frenos que el par disipa.

## La idea

Crawl4RS y Playwright no compiten: son **dos marchas de un mismo motor**.

- **Marcha corta — Crawl4RS (fast HTTP).** Plumífera, determinista, baratísima.
  Cubre el ~90% de la web que no se resiste.
- **Marcha larga — Playwright (navegador real).** Cara pero imparable. Solo
  entra cuando la web se pone dura.

Crawl4RS **ya tiene el embrague**: `FetchMode {fast | browser | auto}` con
auto-escalación al detectar un *challenge* (`looks_like_challenge()`) y un
`Fetcher` conectable (`with_browser_fetcher`). Playwright es la marcha más
larga que le falta para los sitios más agresivos, como un `Fetcher` más.

## La costura (handoff)

Siempre la misma: **Playwright entrega HTML renderizado y/o sesión; Crawl4RS
toma el relevo** para extraer (JSON-LD, CSS, `map`) y para el volumen
determinista. Uno *abre la puerta*, el otro *vacía la habitación*.

## La escalera de escalación (el algoritmo de la etapa)

```
1. fast HTTP (reqwest)                         ← por defecto, barato
2. ¿señal de fallo?  cuerpo vacío/fino · marcadores "requiere JS" ·
   403/429 · HTML de challenge · redirección a login
3. → navegador propio (chromiumoxide)          ← render JS moderado
4. → Playwright (marcha larga)                 ← anti-bot duro · interacción · login
5. handoff: HTML/ sesión vuelven al carril rápido para el volumen
6. (opcional) capa de proxy                    ← reputación de IP
```

Regla de oro: **subir de marcha solo lo justo.** La web deja de ser un muro
binario (puedo / no puedo) y se vuelve un gradiente que el par recorre.

---

## Los frenos, en profundidad

Cada freno: **qué es · cómo lo disipa el par · dónde vive en Crawl4RS · residual honesto.**

### A. Renderizado / JavaScript
- **Qué:** SPA (React/Vue), hidratación tardía, contenido inyectado tras la
  carga, datos en blobs inline (`__NEXT_DATA__`, `window.__INITIAL_STATE__`),
  web components / shadow DOM.
- **Disipación:** el navegador renderiza y entrega el DOM completo. Truco
  barato: muchas veces el dato ya está en un JSON inline → tras render,
  Crawl4RS lo saca sin interacción.
- **En Crawl4RS:** escalón 3 (chromiumoxide) o 4 (Playwright) → extracción normal.
- **Residual:** render 100% cliente con estado ofuscado.

### B. Autenticación
- **Qué:** login por formulario, OAuth/SSO con redirecciones, MFA/OTP, sesión
  por cookie vs token (JWT en localStorage), tokens CSRF.
- **Disipación:** Playwright hace el login *una vez* y exporta `storageState`
  (cookies + localStorage). Crawl4RS lo inyecta para el volumen. Token en
  localStorage → se pasa como cabecera.
- **En Crawl4RS:** feature aditiva `SessionAuth` que el `Fetcher` inyecta
  (cookies/cabeceras) en HTTP y en navegador.
- **Residual:** MFA necesita fuente de OTP (humano/TOTP); sesión atada a
  huella de dispositivo.

### C. Anti-bot / fingerprinting
- **Qué:** huella TLS/JA3 (los clientes HTTP tienen un *handshake* delator),
  huella HTTP/2, huella JS (`navigator.webdriver`, canvas, WebGL, fuentes),
  señales de comportamiento (ratón, ritmo), retos gestionados
  (Cloudflare/Akamai/DataDome/PerimeterX/Turnstile), reputación de IP (rangos
  de datacenter bloqueados).
- **Disipación:** Playwright = motor de navegador real → pasa la huella TLS/JS
  y los retos JS automáticamente. Crawl4RS-fast falla justo aquí: **por eso
  escala.** Parches *stealth* (ocultar `webdriver`) + proxies residenciales
  para la reputación de IP.
- **En Crawl4RS:** la detección `looks_like_challenge()` dispara el escalón 4;
  la capa de proxy es el escalón 6.
- **Residual honesto:** DataDome/PerimeterX/Turnstile detectan **incluso**
  navegadores automatizados por señales de comportamiento y de *headless*. A
  veces requiere residencial + stealth + ritmo humano, y a veces **no hay
  solución limpia.** No lo prometemos.

### D. Interacción para revelar
- **Qué:** scroll infinito, paginación, "cargar más", pestañas/acordeones,
  precio que aparece al hover, banners de cookies/geo que tapan, imágenes
  perezosas (`data-src`).
- **Disipación:** Playwright guioniza la interacción (scroll/click, cerrar
  banners, esperar), y entrega el DOM ya completo a Crawl4RS.
- **En Crawl4RS:** escalón 4; el resultado renderizado entra por el pipeline normal.
- **Residual:** scroll infinito sin fin (¿cuándo parar?); interacción que
  dispara *rate limits*.

### E. Estructura / extracción
- **Qué:** clases ofuscadas (CSS hasheado), precio partido en varios `span`,
  dato **solo en imagen** (precio como PNG), maquetas inconsistentes, variantes A/B.
- **Disipación:** preferir **API interception** (JSON limpio) o
  JSON-LD/microdata sobre selectores frágiles. Y el enganche bonito: **precio
  en imagen → OCR4RS.** Los tres órganos se encuentran (Crawl4RS lee,
  Playwright abre, OCR4RS reconoce lo rasterizado).
- **En Crawl4RS:** extracción JSON-LD / CSS con atributos ya existentes.
- **Residual:** fragilidad por sitio; los cambios de maqueta rompen selectores
  → lo cubre la monitorización de cambios.

### F. Volumen / ritmo / coste
- **Qué:** *rate limiting* (429), baneo de IP por ráfaga, *throttling* por
  cuenta, y el coste de un navegador por página.
- **Disipación:** la escalación de dos marchas (navegador solo cuando hace
  falta), cortesía (retardos, tope de concurrencia), rotación de proxy, caché,
  peticiones condicionales (ETag / If-Modified-Since), reutilización de sesión.
- **En Crawl4RS:** caché (sled) y concurrencia ya existentes; el carril rápido
  es lo que hace el volumen barato.
- **Residual:** un *rate limit* agresivo topa el rendimiento hagas lo que hagas.

### G. Estado / caducidad
- **Qué:** la sesión caduca, tokens rotatorios, CSRF por petición.
- **Disipación:** bucle de re-login al detectar 401 / redirección-a-login (la
  misma detección de challenge generalizada a "sesión perdida"); Playwright
  refresca y el token vuelve al carril rápido.
- **En Crawl4RS:** reutiliza `looks_like_challenge()` extendido a "sesión perdida".
- **Residual:** sesiones muy cortas encarecen el re-login.

### H. Legal / ético (freno no técnico, pero real)
- **Qué:** ToS de cada sitio, `robots.txt`, datos personales (GDPR), abuso.
- **Postura:** la herramienta es neutral; el uso es decisión del operador.
  Recomendado: tus propias cuentas / portales de proveedor, cortesía de ritmo,
  y respetar `robots.txt` donde proceda.

---

## Resumen de la etapa

Dejamos de tener "un scraper" y tenemos un **motor web de dos marchas** que se
adapta a la resistencia de cada sitio, con handoff limpio a la extracción
determinista y — cuando el dato es un ráster — al OCR. La implementación
concreta de esta etapa es un `PlaywrightFetcher` que encaje como marcha larga
en el `Fetcher` de Crawl4RS, más la feature aditiva `SessionAuth`.
