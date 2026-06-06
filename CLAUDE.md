# CLAUDE.md

Guía del repositorio para Claude Code. Idioma del repo: **Fiel positivo (Fiel 2)**;
todos los documentos en español.

## Pseudocódigo — `ModuloEventCore`

Especificación agnóstica al lenguaje de la clase base abstracta de todo módulo
del sistema event-core. Traducible 1:1 a [`event-core/modulo-event-core.js`](./event-core/modulo-event-core.js).

```
ABSTRACTA ModuloEventCore EXTIENDE BaseModule

  CONSTRUCTOR(bus, manifest, logger):
    PRECOND: no instanciar la abstracta directamente
    PRECOND: subclase implementa onLoad y onUnload
    this.bus, this.manifest, this.prefix ← manifest.prefix
    this.estado ← null            // se construye en onLoad (NO Map defensivo)

  // --- Template Method del ciclo de vida ---
  ASÍNCRONO load(context):
    para cada sub en manifest.events.subscribes:   // el loader las cablea
        this.bus.subscribe(sub.name, ruteador(sub.handler), {categoria: sub.categoria})
    await this.onLoad(context)                      // subclase: subs extra + replay canónico
    await this.bus.publish("status", {state:"loaded", modulo:this.name}, {categoria:"status"})

  ABSTRACTO onLoad(context)   // registrar subs; reconstruir estado por REPLAY del bus; jamás disco
  ABSTRACTO onUnload()        // liberar recursos sin persistir fuera del bus

  // --- Helpers que hacen lo correcto fácil y lo incorrecto imposible ---
  ASÍNCRONO emit(entity, verb, payload):
    nombre ← prefix + "." + entity + "." + verb
    SI verb NO ∈ whitelist_verbos: LANZA INVALID_INPUT   // past participle del idioma
    SI faltan campos requeridos(payload): LANZA INVALID_INPUT  // payload completo antes de publish
    await this.bus.publish(nombre, payload, {categoria:"events"})

  sourceModule(event):
    SI no event.source.module: LANZA INVALID_INPUT      // sin fallback a "activo"
    DEVUELVE event.source.module

  ASÍNCRONO io.read(path, project_id):                  // I/O SIEMPRE por el bus
    DEVUELVE await this.bus.mqttRequest("fs","read",{path,project_id})

CASOS LÍMITE:
  - id no resuelto en estado.get(id) → LANZA (fallar ruidoso, no buscar parecido)
  - subclase no implementa onLoad → LANZA al construir
  - verb fuera de whitelist → INVALID_INPUT
```
