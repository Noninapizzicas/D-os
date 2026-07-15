# Contrato del puente Crawl4RS ↔ Playwright — pasada Prisma

> Método: el **molde universal de Prisma** (Enki, `modules/prisma`) aplicado a
> cada herramienta *como un producto*. 5 huecos fijos + "lo objetivo lo
> descompone la IA; lo privado/incierto se marca **ABIERTO**, no se inventa".
> El molde nació del skill `product-capability` (software) → volver a aplicarlo
> a software es devolverlo a su origen. Los ejes/naturalezas de *comercio*
> (tiempo, stock, precio) **no encienden** aquí, y decirlo es fiel al método.
>
> Estado: **propuesta**. El subconjunto `ahora` es implementable ya; el resto
> queda `reservado` (declarado en el contrato, sin código — las "5 líneas de
> más" que multiplican sin obligar a retocar luego).

---

## 1. Crawl4RS — ProductoUniversal

```json
{
  "esquema": "producto-universal-v1 · Prisma",
  "identidad": {
    "que_es": "scraper web determinista (marcha corta)",
    "trabajo_que_resuelve": "URL → datos limpios (markdown/estructura), barato y repetible"
  },
  "restricciones": [
    { "tipo": "verdad_obligatoria", "regla": "nunca inventa contenido; sin config → 503 / errores tipados", "no_negociable": true },
    { "tipo": "factibilidad",       "regla": "no razona en el bucle (no decide clics ni navega flujos)", "no_negociable": true },
    { "tipo": "periodo",            "regla": "cortesía/robots configurable; no martillea", "no_negociable": false }
  ],
  "contrato": {
    "atributos_saber": [
      { "nombre": "url" }, { "nombre": "status" }, { "nombre": "content_type" }, { "nombre": "final_url" }
    ],
    "opciones": [
      { "id": "modo_fetch",   "sub_forma": "variante",     "modo": "ELEGIR_UNO",    "valores": ["fast","browser","auto"] },
      { "id": "formato",      "sub_forma": "variante",     "modo": "ELEGIR_UNO",    "valores": ["markdown","estructurado","html_crudo"] },
      { "id": "extractores",  "sub_forma": "modificacion", "modo": "ELEGIR_VARIOS", "valores": ["json-ld","css+attr","map"] },
      { "id": "busqueda",     "sub_forma": "añadido",      "modo": "ELEGIR_UNO",    "valores": ["searxng"] },
      { "id": "pdf_digital",  "sub_forma": "añadido",      "modo": "ELEGIR_UNO",    "valores": ["→texto"] },
      { "id": "sesion_auth",  "sub_forma": "añadido",      "modo": "LIBRE",         "estado": "RESERVADA", "valores": ["cookies","cabeceras","storageState"] }
    ],
    "estados": ["solicitado","fetched","challenge_detectado","escalar","failed"]
  },
  "ejes_encendidos": { "tiempo": "ninguno", "estado_de_partida_cliente": false, "ciclo": "de_ida", "nota": "ejes de comercio no aplican" },
  "naturalezas": { "stock": "no_aplica", "precio": "no_aplica" },
  "no_objetivos": [
    "no es agente", "no razona clics", "no hace login interactivo",
    "no persiste en BD", "no rompe anti-bot duro por sí solo"
  ],
  "preguntas_abiertas": [
    { "campo": "proxy_residencial", "para": "infra",   "porque": "privado" },
    { "campo": "credenciales_sesion","para": "puente",  "porque": "las aporta el puente, no Crawl4RS" },
    { "campo": "rate_limit_por_sitio","para": "runtime","porque": "no_computable_a_priori" },
    { "campo": "politica_robots",   "para": "operador", "porque": "privado" }
  ],
  "madurez": "listo (núcleo en verde) · sesion_auth reservada · escalación a Playwright reservada"
}
```

## 2. Playwright — ProductoUniversal

```json
{
  "esquema": "producto-universal-v1 · Prisma",
  "identidad": {
    "que_es": "brazo interactivo de navegador real (marcha larga)",
    "trabajo_que_resuelve": "abrir lo que se resiste (JS, login, interacción, anti-bot) y entregar HTML/sesión"
  },
  "restricciones": [
    { "tipo": "verdad_obligatoria", "regla": "no inventa; reporta el fallo real (reto no superado, timeout)", "no_negociable": true },
    { "tipo": "factibilidad",       "regla": "caro (un navegador por sesión) → no es para volumen", "no_negociable": true },
    { "tipo": "retorno",            "regla": "la sesión captada es sensible (credenciales): manejar con cuidado", "no_negociable": true }
  ],
  "contrato": {
    "atributos_saber": [
      { "nombre": "url" }, { "nombre": "status" }, { "nombre": "final_url" }, { "nombre": "tiempo_render" }
    ],
    "opciones": [
      { "id": "render",       "sub_forma": "variante",     "modo": "ELEGIR_UNO",    "valores": ["headless","headed"] },
      { "id": "interactuar",  "sub_forma": "modificacion", "modo": "ELEGIR_VARIOS", "valores": ["click","type","scroll","fill_form","wait"] },
      { "id": "login",        "sub_forma": "añadido",      "modo": "LIBRE",         "valores": ["→ sesion(storageState)"] },
      { "id": "interceptar",  "sub_forma": "añadido",      "modo": "ELEGIR_UNO",    "valores": ["json_api_interna"] },
      { "id": "capturar",     "sub_forma": "añadido",      "modo": "ELEGIR_VARIOS", "valores": ["screenshot","pdf"] },
      { "id": "emular",       "sub_forma": "variante",     "modo": "ELEGIR_VARIOS", "valores": ["dispositivo","geo","idioma"] },
      { "id": "multi_contexto","sub_forma": "añadido",     "modo": "ELEGIR_VARIOS", "valores": ["multi_cuenta"] }
    ],
    "estados": ["navegando","render_ok","reto_presente","reto_superado","sesion_capturada","failed"]
  },
  "ejes_encendidos": { "nota": "ejes de comercio no aplican" },
  "naturalezas": { "stock": "no_aplica", "precio": "no_aplica" },
  "no_objetivos": [
    "no hace extracción determinista a escala", "no es el volumen",
    "no persiste en BD", "no resuelve MFA por sí solo"
  ],
  "preguntas_abiertas": [
    { "campo": "transporte_puente", "para": "arquitectura", "porque": "MCP existente vs HTTP fino — no_resuelto" },
    { "campo": "topologia_docker",  "para": "arquitectura", "porque": "dos contenedores vs embutido — no_resuelto" },
    { "campo": "mfa_otp",           "para": "externo",      "porque": "fuente externa/privada" },
    { "campo": "proxy_residencial", "para": "infra",        "porque": "privado" },
    { "campo": "politica_stealth",  "para": "arquitectura", "porque": "no_resuelto" }
  ],
  "madurez": "la herramienta existe y corre en Docker; el ACOPLE (contrato + transporte) es reservado / no construido"
}
```

## 3. El contrato del puente (derivado de los dos espectros)

Bidireccional. Cada dirección = un verbo con entrada/salida. Cada capacidad
marcada `ahora` (implementar ya) o `reservada` (declarada, sin código).

```jsonc
// contrato-puente-v1
{
  // Dirección 1 — Crawl4RS → Playwright : "ábreme lo que se me resiste"
  "abrir": {
    "in": {
      "url": "string",
      "interactuar?": "[pasos]",        // reservada
      "login?":       "{flujo|selectores}", // reservada
      "interceptar?": "bool",           // reservada
      "emular?":      "{geo,idioma,dispositivo}", // reservada
      "capturar?":    "[screenshot|pdf]"          // reservada
    },
    "out": {
      "html": "string", "final_url": "string", "status": "int",
      "sesion?":      "storageState",   // reservada
      "intercepted?": "[json]",         // reservada
      "artefactos?":  "[...]",          // reservada
      "fallo?": "{tipo: reto_no_superado|timeout, motivo}"
    },
    "ahora": "abrir(url) -> {html, final_url, status}"   // render simple
  },

  // Dirección 2 — Playwright → Crawl4RS : "vacía la habitación"
  "extraer": {
    "in":  { "fuente": "html|url", "formato": "markdown|estructurado", "extractores?": "[...]", "selectores?": "[...]" },
    "out": { "resultado": "markdown|estructurado", "metadatos": "{status, content_type, final_url}" },
    "ahora": "extraer(html, {formato: markdown}) -> {markdown, metadatos}"
  },
  "volumen": {   // reservada
    "in":  { "urls": "[string]", "sesion?": "storageState", "modo": "fast|auto" },
    "out": "[resultado]"
  },

  // Transversal
  "sesion":     "storageState = cookies + localStorage  (objeto de intercambio clave)",
  "transporte": "ABIERTO: MCP existente | HTTP fino  (es cable, no contrato)",
  "topologia":  "ABIERTO: dos contenedores | embutido",
  "honestidad": "el fallo se REPORTA, no se inventa (verdad_obligatoria de ambos)",
  "version":    "v1; capacidades = ahora | reservada"
}
```

### Preguntas abiertas del contrato (no se inventan)
- **Transporte y topología Docker** (MCP vs HTTP; dos contenedores vs embutido).
- **Forma exacta del login** (guion por selectores vs blueprint LLM).
- **Custodia de la sesión** (¿la guarda el puente? ¿caducidad/refresco?).
- **MFA/OTP** (fuente externa).

### Veredicto de madurez
**Propuesta lista para decidir.** El subconjunto `ahora` — `abrir(url)→{html}`
y `extraer(html)→{markdown}` — es implementable de inmediato y cierra la
capacidad de extracción de punta a punta en su forma mínima. Todo lo demás
queda **reservado**: declarado en el contrato, para rellenar sin rediseñar.
