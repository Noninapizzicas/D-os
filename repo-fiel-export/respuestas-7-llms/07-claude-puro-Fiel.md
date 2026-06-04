# claude puro Fiel

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.


---
## Mensaje 1 (2026-06-03T09:11:51)

no hace falta más acabas de descifrarlo tú todo llama muy acertadamente lo que sí que te planteo es que esto a lo mejor un melón grande habrá que repartirlo en tajadas para ir comiéndolo poco a poco pero la ideal acaba de sintetizar tu correctamente si no la plasmas fielmente degenerará y volveremos otra vez a tener que volver al mismo. punto y aquí es donde yo te hago una puerta porque a lo mejor te puede ayudar a mí me gusta reinventar cosas algunas se me equivoco otras veces pues que acierte pero se puede mírar yo entiendo la el doble lenguaje que lleváis la LLM estáis pensando en un sistema y lo comuniqué con nosotros con otros sistemas totalmente distintos vale y si acercamos para este trabajo un lenguaje de mas  intermedio que no te ofrezca matices donde puedas divagar no podemos utilizar herramienta o patrones que no estructuran de mejor forma nuestro pensamiento en vez de utilizar skill o contrato en MD utilizarlo en JSON con lenguaje pseudocódigo con estructura oop

---
## Mensaje 2 (2026-06-03T14:44:29)

{
  "_meta": {
    "documento_id": "experimento-3coops-001-respuesta-fiel",
    "version": "0.1.0",
    "creado": "2026-06-03",
    "language": "es-ES",
    "supersedes_si_aplica": null,
    "respuesta_a": "arquitectura/decisiones/propuestas/_prompt-disenyo-lenguaje-3coops-v0.1.json",
    "doc_corto": "Definicion v0.1.0 del lenguaje Fiel producida bajo el prompt cocinado. Una sola pieza: la definicion universal del lenguaje, sin caso de uso aplicativo mezclado."
  },

  "name": "Fiel",

  "philosophy_breve": "Fiel canoniza disciplina arquitectonica. JSON es contenedor, OOP es modelo conceptual, pseudocodigo estructurado es logica. El nombre nombra dos cosas: fidelidad al paradigma event-core y el fiel de la balanza que calibra la tendencia del LLM. Un documento Fiel modela tambien la deriva que el corpus le ensena al LLM, porque un lenguaje para LLMs que ignora la tendencia del LLM produce documentos correctos que se ejecutan torcidos. El pseudocodigo es ley: pasos discretos, no prosa.",

  "componentes": [
    {
      "id": "code",
      "descripcion_corta": "JSON estricto validable por AJV strict como unico contenedor sintactico. Sin sintaxis inventada fuera de JSON.",
      "constraints": [
        "additionalProperties false en cada objeto",
        "cada string field declara maxLength o pattern",
        "sin conectores discursivos en valores"
      ]
    },
    {
      "id": "OOP",
      "descripcion_corta": "Modelo conceptual via clase, clase abstracta, properties, constructor, methods, extends, invariantes concretas, patrones prohibidos.",
      "constraints": [
        "discriminador type explicito en cada instancia",
        "extends referencia clase existente o null",
        "constructor sin I/O"
      ]
    },
    {
      "id": "pseudo_code",
      "descripcion_corta": "Logica algoritmica como array de pasos discretos con shape PseudoStep. No strings raw multilinea.",
      "constraints": [
        "cada paso es objeto { action, target, condition, value }",
        "action pertenece al enum de acciones canonicas",
        "maxItems acotado por contexto del paso"
      ]
    }
  ],

  "primitivas_canonicas": [
    {
      "name": "PseudoStep",
      "type": "primitiva",
      "shape_json": {
        "action": "string matches /^[a-z_]+$/ in enum acciones_canonicas",
        "target": "string maxLength 80 | null",
        "condition": "string maxLength 120 | null",
        "value": "string maxLength 120 | number | boolean | null"
      },
      "acciones_canonicas": [
        "assign",
        "check",
        "await",
        "publish",
        "publish_and_wait",
        "return",
        "throw",
        "log",
        "require_clarification",
        "regenerate",
        "iterate",
        "noop"
      ]
    },
    {
      "name": "Param",
      "type": "primitiva",
      "shape_json": {
        "name": "string matches /^[a-z][A-Za-z0-9]*$/",
        "type_expression": "string maxLength 50",
        "required": "boolean"
      }
    },
    {
      "name": "Property",
      "type": "primitiva",
      "shape_json": {
        "name": "string matches /^[a-z][A-Za-z0-9]*$/",
        "type_expression": "string maxLength 50",
        "visibility": "enum [private, protected, public]",
        "authority_source": "enum [bus, local_constant, derived]"
      }
    },
    {
      "name": "Constructor",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"constructor\"",
        "params": "array<Param>",
        "pseudo_code": "array<PseudoStep> maxItems 15",
        "io_forbidden": "literal true"
      }
    },
    {
      "name": "Method",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"method\"",
        "name": "string matches /^[a-z][A-Za-z0-9]*$/",
        "visibility": "enum [public, private, protected]",
        "params": "array<Param>",
        "returns": "string maxLength 50",
        "pseudo_code": "array<PseudoStep> maxItems 30",
        "throws": "array<error_code>"
      }
    },
    {
      "name": "Invariant",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"invariant\"",
        "expression": "PseudoStep with action check",
        "applies_to": "string maxLength 60 (ClassName | MethodName | global)"
      }
    },
    {
      "name": "ForbiddenPattern",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"forbidden_pattern\"",
        "pattern_regex": "string PCRE valid maxLength 200",
        "reason": "string maxLength 200",
        "alternative_pseudo_code": "array<PseudoStep> maxItems 8"
      }
    },
    {
      "name": "FileLineReference",
      "type": "primitiva",
      "shape_json": {
        "archivo": "string matches /^[a-z0-9_/.-]+$/ maxLength 160",
        "linea_aprox": "integer >= 0 | null",
        "drift_id": "string matches /^(drift|audit)_[a-z0-9_]+$/ | null"
      }
    },
    {
      "name": "Tendencia",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"tendencia\"",
        "id": "string matches /^T[0-9]+_[a-z_]+$/",
        "creencia_aprendida_corpus": "string maxLength 200",
        "verdad_paradigma_que_la_contradice": "string maxLength 200",
        "patron_codigo_a_evitar_regex": "array<string PCRE valid>",
        "alternativa_canonica_pseudo_code": "array<PseudoStep> maxItems 6",
        "caso_testigo": "FileLineReference"
      }
    },
    {
      "name": "AbstractClass",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"abstract_class\"",
        "name": "string matches /^[A-Z][A-Za-z0-9]*$/",
        "extends": "string | null",
        "properties": "array<Property>",
        "constructor": "Constructor | null",
        "methods": "array<Method>",
        "concrete_invariants": "array<Invariant>",
        "forbidden_patterns": "array<ForbiddenPattern>"
      }
    },
    {
      "name": "Class",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"class\"",
        "name": "string matches /^[A-Z][A-Za-z0-9]*$/",
        "extends": "string | null",
        "properties": "array<Property>",
        "constructor": "Constructor | null",
        "methods": "array<Method>",
        "concrete_invariants": "array<Invariant>",
        "forbidden_patterns": "array<ForbiddenPattern>"
      }
    },
    {
      "name": "AutoAuditProtocol",
      "type": "primitiva",
      "shape_json": {
        "type": "literal \"auto_audit_protocol\"",
        "runs": "enum [after_generation_before_return]",
        "steps": "array<PseudoStep> maxItems 12",
        "on_violation": "enum [regenerate_section, return_requires_clarification]"
      }
    }
  ],

  "AbstractClass_ejemplar_que_modela_caso_de_uso_real": {
    "type": "abstract_class",
    "name": "ContratoTransversal",
    "extends": null,
    "properties": [
      { "name": "id", "type_expression": "string", "visibility": "public", "authority_source": "local_constant" },
      { "name": "version", "type_expression": "semver", "visibility": "public", "authority_source": "local_constant" },
      { "name": "objetivo", "type_expression": "string", "visibility": "public", "authority_source": "local_constant" },
      { "name": "principios", "type_expression": "array<Principio>", "visibility": "public", "authority_source": "local_constant" },
      { "name": "prohibido", "type_expression": "array<ForbiddenPattern>", "visibility": "public", "authority_source": "local_constant" },
      { "name": "validacionesCross", "type_expression": "array<CrossCheck>", "visibility": "public", "authority_source": "derived" }
    ],
    "constructor": {
      "type": "constructor",
      "params": [
        { "name": "id", "type_expression": "string", "required": true },
        { "name": "version", "type_expression": "semver", "required": true }
      ],
      "pseudo_code": [
        { "action": "assign", "target": "this.id", "condition": null, "value": "id" },
        { "action": "assign", "target": "this.version", "condition": null, "value": "version" }
      ],
      "io_forbidden": true
    },
    "methods": [
      {
        "type": "method",
        "name": "validateAgainstRepo",
        "visibility": "public",
        "params": [],
        "returns": "ValidationReport",
        "pseudo_code": [
          { "action": "iterate", "target": "this.validacionesCross", "condition": null, "value": null },
          { "action": "check", "target": "check.regla", "condition": "por cada crossCheck", "value": null },
          { "action": "return", "target": "report", "condition": null, "value": null }
        ],
        "throws": ["INVALID_INPUT"]
      },
      {
        "type": "method",
        "name": "deriveOutput",
        "visibility": "public",
        "params": [],
        "returns": "OutputJson",
        "pseudo_code": [
          { "action": "publish_and_wait", "target": "this.bus", "condition": null, "value": "fs.read.request" },
          { "action": "assign", "target": "output", "condition": null, "value": "extraido de inputs" },
          { "action": "return", "target": "output", "condition": null, "value": null }
        ],
        "throws": ["INVALID_INPUT"]
      }
    ],
    "concrete_invariants": [
      { "type": "invariant", "expression": { "action": "check", "target": "this.id === filename sin sufijo .contract.json", "condition": null, "value": null }, "applies_to": "ContratoTransversal" },
      { "type": "invariant", "expression": { "action": "check", "target": "this.version", "condition": "matches semver", "value": null }, "applies_to": "ContratoTransversal" },
      { "type": "invariant", "expression": { "action": "check", "target": "this.prohibido.length", "condition": "> 0", "value": null }, "applies_to": "ContratoTransversal" }
    ],
    "forbidden_patterns": [
      {
        "type": "forbidden_pattern",
        "pattern_regex": "\\b(aunque|salvo que|en general|sin embargo|por otro lado)\\b",
        "reason": "Conector discursivo es sintoma de prosa libre infiltrada en contrato",
        "alternative_pseudo_code": [
          { "action": "assign", "target": "caso_borde", "condition": "el matiz justifica el conector", "value": "{ condicion, comportamiento }" }
        ]
      }
    ]
  },

  "tendencias_del_llm": [
    {
      "type": "tendencia",
      "id": "T1_cache_defensiva",
      "creencia_aprendida_corpus": "Memorizar evita llamadas extra al bus y mejora performance",
      "verdad_paradigma_que_la_contradice": "Bus es fuente unica de autoridad; memorizar duplica autoridad",
      "patron_codigo_a_evitar_regex": ["this\\._?cache\\s*=\\s*new Map", "this\\.\\w+PerProject\\s*=\\s*new Map"],
      "alternativa_canonica_pseudo_code": [
        { "action": "publish_and_wait", "target": "this.bus", "condition": null, "value": "{ eventType, payload }" },
        { "action": "return", "target": "result", "condition": null, "value": null }
      ],
      "caso_testigo": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 28, "drift_id": "drift_cache_defensiva" }
    },
    {
      "type": "tendencia",
      "id": "T2_fallback_silencioso_identidad",
      "creencia_aprendida_corpus": "Si el id no resuelve busco algo parecido para no romper flujo",
      "verdad_paradigma_que_la_contradice": "Id no resuelto es bug; fallar ruidoso expone el desencuentro",
      "patron_codigo_a_evitar_regex": ["resolveTo\\w*Fallback", "for.*of this\\.\\w+\\.keys.*return"],
      "alternativa_canonica_pseudo_code": [
        { "action": "check", "target": "cache.has(id)", "condition": null, "value": null },
        { "action": "throw", "target": "INVALID_INPUT", "condition": "!cache.has(id)", "value": null }
      ],
      "caso_testigo": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 62, "drift_id": "drift_fallback_silencioso" }
    },
    {
      "type": "tendencia",
      "id": "T3_bypass_filesystem",
      "creencia_aprendida_corpus": "fs.readdir y fs.readFile son APIs estandar; usarlas es eficiente",
      "verdad_paradigma_que_la_contradice": "filesystem es modulo dueno del scope; bus mantiene la autoridad",
      "patron_codigo_a_evitar_regex": ["require\\(['\"]fs['\"]\\)", "await fs\\.(readdir|readFile|writeFile)"],
      "alternativa_canonica_pseudo_code": [
        { "action": "publish_and_wait", "target": "this.bus", "condition": null, "value": "fs.read.request" },
        { "action": "return", "target": "result", "condition": null, "value": null }
      ],
      "caso_testigo": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 1133, "drift_id": "drift_bypass_filesystem" }
    },
    {
      "type": "tendencia",
      "id": "T4_catch_tragador",
      "creencia_aprendida_corpus": "try catch vacio para que el flujo no rompa",
      "verdad_paradigma_que_la_contradice": "Error oculto se manifiesta despues como inconsistencia inexplicable",
      "patron_codigo_a_evitar_regex": ["catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}"],
      "alternativa_canonica_pseudo_code": [
        { "action": "log", "target": "logger.error", "condition": "catch(err)", "value": "mod.op.failed" },
        { "action": "throw", "target": "err", "condition": null, "value": null }
      ],
      "caso_testigo": { "archivo": "modules/filesystem/index.js", "linea_aprox": 141, "drift_id": "drift_catch_tragador" }
    },
    {
      "type": "tendencia",
      "id": "T5_sobreescribir_estado_al_recargar",
      "creencia_aprendida_corpus": "Al cargar normalizo campos como activo true para tener estado consistente",
      "verdad_paradigma_que_la_contradice": "Estado persistido es la verdad; sobreescribir en carga pierde el estado real",
      "patron_codigo_a_evitar_regex": ["\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}"],
      "alternativa_canonica_pseudo_code": [
        { "action": "assign", "target": "item", "condition": "sin sobreescribir", "value": "{ ...rawData }" }
      ],
      "caso_testigo": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 1167, "drift_id": "drift_sobreescribir_estado" }
    },
    {
      "type": "tendencia",
      "id": "T6_completar_campos_que_pseudocodigo_no_pide",
      "creencia_aprendida_corpus": "El campo esta vacio; pongo un valor razonable",
      "verdad_paradigma_que_la_contradice": "pseudocodigo es ley; nowISO es nowISO; valores no especificados no se inventan",
      "patron_codigo_a_evitar_regex": ["created_at:\\s*['\"][0-9]{4}-", "version:\\s*2\\s*,"],
      "alternativa_canonica_pseudo_code": [
        { "action": "assign", "target": "created_at", "condition": null, "value": "nowISO()" },
        { "action": "assign", "target": "version", "condition": "el paso no lo especifica", "value": 1 }
      ],
      "caso_testigo": { "archivo": "audit_2026-06-02_carta_catalogo_activo", "linea_aprox": null, "drift_id": "audit_carta_catalogo_activo" }
    },
    {
      "type": "tendencia",
      "id": "T7_handoff_prematuro",
      "creencia_aprendida_corpus": "Eventual consistency es legitimo; publico al empezar la accion",
      "verdad_paradigma_que_la_contradice": "Publica cuando el cambio aterrizo o con payload completo para que el consumer no lea disco",
      "patron_codigo_a_evitar_regex": ["this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await.*write"],
      "alternativa_canonica_pseudo_code": [
        { "action": "await", "target": "write", "condition": null, "value": null },
        { "action": "publish", "target": "foo.creada", "condition": "tras aterrizar el cambio", "value": "payloadCompleto" }
      ],
      "caso_testigo": { "archivo": "audit_2026-06-02_flujo_carta_manager", "linea_aprox": null, "drift_id": "audit_flujo_carta_manager" }
    },
    {
      "type": "tendencia",
      "id": "T8_deducir_terminos_nuevos_del_contexto_inmediato",
      "creencia_aprendida_corpus": "Cuando el humano usa termino tecnico nuevo lo deduzco del contexto reciente",
      "verdad_paradigma_que_la_contradice": "Deducir mal contamina el documento; preguntar es mas barato que reescribir",
      "patron_codigo_a_evitar_regex": ["// asumir significado sin verificar"],
      "alternativa_canonica_pseudo_code": [
        { "action": "check", "target": "termino_existe_en_repo", "condition": "termino_nuevo", "value": null },
        { "action": "require_clarification", "target": "termino_nuevo", "condition": "!termino_existe_en_repo", "value": null }
      ],
      "caso_testigo": { "archivo": "arquitectura/decisiones/propuestas/_experimento-3coops-001.json", "linea_aprox": 8, "drift_id": "drift_deducir_termino_3coops" }
    }
  ],

  "auto_audit_protocol": {
    "type": "auto_audit_protocol",
    "runs": "after_generation_before_return",
    "steps": [
      { "action": "check", "target": "output vs forbidden_patterns_meta", "condition": "tras generar, antes de devolver", "value": null },
      { "action": "check", "target": "cada PseudoStep.action", "condition": "pertenece al enum acciones_canonicas", "value": null },
      { "action": "check", "target": "cada string field", "condition": "respeta maxLength o pattern", "value": null },
      { "action": "check", "target": "las 8 tendencias T1..T8", "condition": "presentes", "value": null },
      { "action": "regenerate", "target": "seccion_violada", "condition": "violacion detectada", "value": null },
      { "action": "return", "target": "output", "condition": "sin violaciones", "value": null }
    ],
    "on_violation": "regenerate_section"
  },

  "output_rules_para_LLM_que_use_el_lenguaje": [
    { "id": "OR1", "regla": "Responde solo con JSON valido por AJV strict. Sin Markdown fuera de campos doc_corto." },
    { "id": "OR2", "regla": "Todo pseudocodigo es array de PseudoStep. Nunca string raw multilinea." },
    { "id": "OR3", "regla": "Declara discriminador type en cada instancia de primitiva." },
    { "id": "OR4", "regla": "Si un termino tecnico es nuevo y no consta en repo, devuelve requires_clarification y no inventes." },
    { "id": "OR5", "regla": "Ejecuta auto_audit_protocol antes de cerrar. Si detectas violacion, regenera la seccion." }
  ],

  "forbidden_patterns_meta": [
    {
      "type": "forbidden_pattern",
      "pattern_regex": "\\b(aunque|salvo que|en general|sin embargo|por otro lado)\\b",
      "reason": "Conector discursivo es sintoma de prosa libre infiltrada",
      "alternative_pseudo_code": [
        { "action": "assign", "target": "sub_objeto_estructurado", "condition": "el matiz justifica el conector", "value": "{ condicion, comportamiento }" }
      ]
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "[a-z]{30,}",
      "reason": "Token de 30+ letras seguidas es parrafo concatenado o terminologia inventada",
      "alternative_pseudo_code": [
        { "action": "assign", "target": "campos_discretos", "condition": null, "value": "descomponer" }
      ]
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "\"pseudo_code\"\\s*:\\s*\"",
      "reason": "pseudo_code como string es violacion del shape PseudoStep",
      "alternative_pseudo_code": [
        { "action": "assign", "target": "pseudo_code", "condition": null, "value": "array<PseudoStep>" }
      ]
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "definicion_del_lenguaje\\s*[+]\\s*caso_de_uso",
      "reason": "Mezclar definicion universal con caso particular pierde universalidad",
      "alternative_pseudo_code": [
        { "action": "assign", "target": "documento_de_uso", "condition": null, "value": "referencia a la definicion, no la repite" }
      ]
    }
  ],

  "ejemplo_completo": {
    "type": "class",
    "name": "NamingContract",
    "extends": "ContratoTransversal",
    "properties": [
      { "name": "id", "type_expression": "string", "visibility": "public", "authority_source": "local_constant" },
      { "name": "verbosCanonicos", "type_expression": "array<string>", "visibility": "public", "authority_source": "local_constant" }
    ],
    "constructor": {
      "type": "constructor",
      

---
## Mensaje 3 (2026-06-04T02:06:17)

<task-notification>
<task-id>aa2568855aa723adc</task-id>
<tool-use-id>toolu_01Q2zAcsyBFF19zQvDZgRyHp</tool-use-id>
<output-file>/tmp/claude-0/-home-user-2enki/7919e2eb-f0de-452c-99c6-2235a48f005b/tasks/aa2568855aa723adc.output</output-file>
<status>completed</status>
<summary>Agent "Notario plasma pizzepos productos" completed</summary>
<result>Now I'll write the final plasmation as pure JSON.

{
  "_meta": {
    "documento_id": "pizzepos-productos-disciplina",
    "version": "1.0.0",
    "creado": "2026-06-04",
    "language": "es-ES",
    "lenguaje": "Fiel",
    "lenguaje_definicion_ref": "arquitectura/decisiones/propuestas/_fiel-v0.1.0.json",
    "extiende_contrato_ref": "arquitectura/decisiones/propuestas/_contrato-modulo-event-core-disciplina-via-notario.json",
    "fuente_cocinado_ref": "arquitectura/decisiones/propuestas/_arranque-modulo-event-core-disciplina.json#tajada_2",
    "auditoria_modulo_ref": "arquitectura/auditoria/_outputs/modulo-completo/pizzepos__productos.json",
    "manifest_ref": "modules/pizzepos/productos/module.json",
    "codigo_ref": "modules/pizzepos/productos/index.js",
    "plasmado_por": "agente notario segunda prueba",
    "intencion": "Plasma pizzepos/productos como ConcreteClass que extiende ModuloEventCore y documenta drift contra cinco tendencias confirmadas por lectura del codigo fuente. Es documento testigo de drift, no documento de disciplina."
  },

  "type": "concrete_class",
  "name": "PizzeposProductos",
  "extends": "ModuloEventCore",

  "properties": [
    {
      "type": "property",
      "name": "productosPerProject",
      "type_expression": "Map&lt;projectId, Map&lt;productoId, Producto&gt;&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "categoriasPerProject",
      "type_expression": "Map&lt;projectId, Map&lt;categoriaId, Categoria&gt;&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "menusPendientes",
      "type_expression": "Map&lt;menuId, object&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "mappingCanalesPerProject",
      "type_expression": "Map&lt;projectId, object&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "projectPaths",
      "type_expression": "Map&lt;projectId, string&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "pendingProjectRequests",
      "type_expression": "Map&lt;requestId, PendingRequest&gt;",
      "visibility": "private",
      "nullable": false,
      "default": "new Map()"
    },
    {
      "type": "property",
      "name": "storageSection",
      "type_expression": "string",
      "visibility": "private",
      "nullable": false,
      "default": "pizzepos"
    },
    {
      "type": "property",
      "name": "uiHandler",
      "type_expression": "UiHandler | null",
      "visibility": "protected",
      "nullable": true,
      "default": null
    },
    {
      "type": "property",
      "name": "eventBus",
      "type_expression": "EventBus",
      "visibility": "protected",
      "nullable": false,
      "default": null
    },
    {
      "type": "property",
      "name": "version",
      "type_expression": "string",
      "visibility": "public",
      "nullable": false,
      "default": "4.0.0"
    }
  ],

  "bus_connection": {
    "type": "bus_connection",
    "publishes": [
      {
        "event_type": "producto.creado",
        "shape_ref": "{ project_id, producto_id, correlation_id, timestamp }",
        "consumed_by": ["comandero", "pedidos"],
        "produces_response_event": null
      },
      {
        "event_type": "producto.actualizado",
        "shape_ref": "{ project_id, producto_id, cambios, correlation_id, timestamp }",
        "consumed_by": ["comandero", "pedidos"],
        "produces_response_event": null
      },
      {
        "event_type": "producto.eliminado",
        "shape_ref": "{ project_id, producto_id, motivo, correlation_id, timestamp }",
        "consumed_by": ["comandero", "pedidos"],
        "produces_response_event": null
      },
      {
        "event_type": "catalogo.actualizado",
        "shape_ref": "{ project_id, productos, source, correlation_id, timestamp }",
        "consumed_by": ["comandero", "pedidos"],
        "produces_response_event": null
      },
      {
        "event_type": "menu.generado",
        "shape_ref": "shape no documentado en manifest; emitido en index.js:1151 no declarado",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "tarifas.config.solicitada",
        "shape_ref": "{ project_id, tipo, correlation_id, timestamp }",
        "consumed_by": ["tarifas"],
        "produces_response_event": "tarifas.config.actualizada"
      }
    ],
    "listens": [
      {
        "event_type": "carta.actualizada",
        "shape_ref": "carta entera embebida en payload",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "carta.editada",
        "shape_ref": "carta entera embebida en payload",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "carta.borrada",
        "shape_ref": "{ carta_id, project_id }",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "tarifas.config.actualizada",
        "shape_ref": "{ project_id, mapping_canales }",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "project.activated",
        "shape_ref": "{ project_id, base_path }",
        "consumed_by": null,
        "produces_response_event": null
      },
      {
        "event_type": "project.get.response",
        "shape_ref": "{ request_id, success, project }",
        "consumed_by": null,
        "produces_response_event": null
      }
    ],
    "publishes_and_waits": [
      {
        "request_event": {
          "event_type": "project.get.request",
          "shape_ref": "{ request_id, project_id }",
          "consumed_by": ["project-manager"],
          "produces_response_event": "project.get.response"
        },
        "response_event": {
          "event_type": "project.get.response",
          "shape_ref": "{ request_id, success, project }",
          "consumed_by": null,
          "produces_response_event": null
        },
        "timeout_ms_default": 5000
      }
    ],
    "constraints": [
      {
        "type": "invariant",
        "expression": "menu.generado y project.get.request publicados sin declaracion en module.json",
        "applies_to": "PizzeposProductos"
      }
    ]
  },

  "enforcedTendencies": [
    "T1_cache_defensiva",
    "T2_fallback_silencioso_identidad",
    "T3_bypass_filesystem",
    "T5_sobreescribir_estado_al_recargar",
    "T7_handoff_prematuro"
  ],

  "violations": [
    {
      "tendencia_id": "T1_cache_defensiva",
      "evidencia": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 28,
        "drift_id": "drift_cache_defensiva"
      },
      "snippet_real_del_codigo": "this.productosPerProject = new Map(); this.categoriasPerProject = new Map(); this.menusPendientes = new Map(); this.mappingCanalesPerProject = new Map();",
      "severidad": "error",
      "descripcion_corta": "Cuatro Maps per-project mantenidos como estado propio rehidratado por handlers de carta. Reconstruccion en onLoad delegada a publish auto de carta.actualizada que puede no llegar"
    },
    {
      "tendencia_id": "T2_fallback_silencioso_identidad",
      "evidencia": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 62,
        "drift_id": "drift_fallback_silencioso_identidad"
      },
      "snippet_real_del_codigo": "resolveToActiveProject(projectId) { if (this.productosPerProject.has(projectId) &amp;&amp; this.productosPerProject.get(projectId).size &gt; 0) return projectId; for (const [pid, prods] of this.productosPerProject) { if (prods.size &gt; 0) { this.logger.debug('productos.resolve_fallback', { requested: projectId, resolved: pid }); return pid; } } return projectId; }",
      "severidad": "error",
      "descripcion_corta": "Si el project_id no tiene datos, itera el Map y devuelve el primero con productos. Repetido inline en handleCartaCompleta lineas 884-891. Convierte tools multi-tenant en single-tenant accidental"
    },
    {
      "tendencia_id": "T3_bypass_filesystem",
      "evidencia": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 12,
        "drift_id": "drift_bypass_filesystem"
      },
      "snippet_real_del_codigo": "const fs = require('fs').promises; ... await fs.readdir(cartasDir); const cartaData = await fs.readFile(cartaPath, 'utf8'); await fs.mkdir(productosDir, { recursive: true }); await fs.writeFile(catalogPath, JSON.stringify(catalog, null, 2), 'utf-8');",
      "severidad": "error",
      "descripcion_corta": "Importa fs directo en linea 12. Lecturas en linea 1133 y 1138. Escrituras en linea 1011 y 1026. Modulo filesystem dueño del scope queda bypassed en las dos direcciones"
    },
    {
      "tendencia_id": "T5_sobreescribir_estado_al_recargar",
      "evidencia": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 1167,
        "drift_id": "drift_sobreescritura_estado_al_recargar"
      },
      "snippet_real_del_codigo": "productosMap.set(prod.id, { ...prod, activo: true, carta_source: carta.meta?.id || file.replace('.json', ''), loaded_at: new Date().toISOString() }); ... categoriasMap.set(cat.id, { ...cat, activa: true });",
      "severidad": "error",
      "descripcion_corta": "Al cargar desde disco fuerza activo:true en productos linea 1167 y activa:true en categorias linea 1145. Pierde el estado persistido si la carta serializa producto desactivado"
    },
    {
      "tendencia_id": "T7_handoff_prematuro",
      "evidencia": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 681,
        "drift_id": "drift_handoff_prematuro"
      },
      "snippet_real_del_codigo": "await this.publishProductoActualizado(project_id, id, cambios); await this.persistCatalog(project_id);",
      "severidad": "error",
      "descripcion_corta": "Publica producto.actualizado en linea 681 y luego llama persistCatalog en linea 684. Patron repetido en linea 349 vs 352 catalogo.actualizado antes de persistCatalog y linea 713 vs 716 producto.eliminado antes de persistCatalog. Consumer que reaccione leyendo disco encuentra estado pre-mutacion"
    }
  ],

  "forbidden_patterns_aplicables": [
    {
      "type": "forbidden_pattern",
      "pattern_regex": "this\\._?cache\\s*=\\s*new Map",
      "reason": "heredado de ModuloEventCore; matchea this.productosPerProject = new Map y this.categoriasPerProject = new Map en linea 28-29 via variante PerProject",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "call", "target": "this.eventBus.publishAndWait", "condition": null, "value": "{ eventType: 'fs.read.request', payload: { project_id, path } }" },
        { "type": "pseudo_step", "action": "assign", "target": "result", "condition": null, "value": "await call_result" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "require\\(['\"]fs['\"]\\)",
      "reason": "heredado de ModuloEventCore; matchea linea 12 require('fs').promises",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.read.request", "condition": null, "value": "{ project_id, path }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.read.response", "condition": "correlation_id matches request", "value": null }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "resolveTo\\w*Fallback|resolveToActive\\w*",
      "reason": "heredado de ModuloEventCore; matchea resolveToActiveProject en linea 62",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "branch", "target": "if_not_cache_has", "condition": "!this.productosPerProject.has(projectId)", "value": null },
        { "type": "pseudo_step", "action": "throw", "target": "INVALID_INPUT", "condition": null, "value": "project_id no resuelto" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}",
      "reason": "heredado de ModuloEventCore; matchea linea 1167 con productosMap.set y linea 1145 con activa:true",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "assign", "target": "item", "condition": null, "value": "{ ...rawData }" },
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "sin sobreescribir campos persistidos" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await\\s+\\w*\\.?write",
      "reason": "heredado de ModuloEventCore; matchea linea 681 publishProductoActualizado seguida en linea 684 de persistCatalog que internamente hace fs.writeFile",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.write.request", "condition": null, "value": "{ path, content }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.write.response", "condition": "correlation_id matches request", "value": null },
        { "type": "pseudo_step", "action": "emit", "target": "producto.actualizado", "condition": "after_write_landed", "value": "{ project_id, producto_id, cambios }" }
      ],
      "severidad": "error"
    }
  ],

  "invariants": [
    {
      "type": "invariant",
      "expression": "this.productosPerProject y this.categoriasPerProject son Map propios mantenidos por handlers de carta",
      "applies_to": "PizzeposProductos"
    },
    {
      "type": "invariant",
      "expression": "publish de mutacion ocurre antes de aterrizar el cambio en disco en lineas 349, 681, 713",
      "applies_to": "PizzeposProductos"
    },
    {
      "type": "invariant",
      "expression": "estado persistido es sobrescrito con activo:true al cargar desde disco en linea 1167",
      "applies_to": "PizzeposProductos"
    }
  ],

  "methods": [
    {
      "type": "method",
      "name": "resolveToActiveProject",
      "visibility": "public",
      "params": [
        { "name": "projectId", "type_expression": "string", "required": true, "default": null }
      ],
      "returns": "string",
      "pseudo_code": [
        { "type": "pseudo_step", "action": "branch", "target": "if_project_tiene_datos", "condition": "this.productosPerProject.has(projectId) AND size &gt; 0", "value": null },
        { "type": "pseudo_step", "action": "return", "target": "projectId", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "loop", "target": "for_each_pid_en_map", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "branch", "target": "if_pid_tiene_productos", "condition": "prods.size &gt; 0", "value": null },
        { "type": "pseudo_step", "action": "log", "target": "this.logger.debug", "condition": null, "value": "{ event: 'productos.resolve_fallback', requested, resolved }" },
        { "type": "pseudo_step", "action": "return", "target": "pid_fallback", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "return", "target": "projectId_original", "condition": null, "value": null }
      ],
      "throws": []
    },
    {
      "type": "method",
      "name": "loadCartaFromProject",
      "visibility": "public",
      "params": [
        { "name": "project_id", "type_expression": "string", "required": true, "default": null }
      ],
      "returns": "{ productos: int, categorias: int }",
      "pseudo_code": [
        { "type": "pseudo_step", "action": "call", "target": "this.resolveStoragePath", "condition": null, "value": "project_id" },
        { "type": "pseudo_step", "action": "fetch", "target": "fs", "condition": null, "value": "readdir(cartasDir)" },
        { "type": "pseudo_step", "action": "loop", "target": "for_each_json_file", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "fetch", "target": "fs", "condition": null, "value": "readFile(cartaPath)" },
        { "type": "pseudo_step", "action": "loop", "target": "for_each_categoria_en_carta", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "store", "target": "categoriasMap", "condition": null, "value": "set(cat.id, { ...cat, activa: true })" },
        { "type": "pseudo_step", "action": "loop", "target": "for_each_producto_en_carta", "condition": null, "value": null },
        { "type": "pseudo_step", "action": "store", "target": "productosMap", "condition": null, "value": "set(prod.id, { ...prod, activo: true, carta_source, loaded_at })" }
      ],
      "throws": ["INVALID_INPUT", "RESOURCE_NOT_FOUND"]
    },
    {
      "type": "method",
      "name": "persistCatalog",
      "visibility": "public",
      "params": [
        { "name": "project_id", "type_expression": "string", "required": true, "default": null }
      ],
      "returns": "void",
      "pseudo_code": [
        { "type": "pseudo_step", "action": "call", "target": "this.resolveStoragePath", "condition": null, "value": "project_id" },
        { "type": "pseudo_step", "action": "fetch", "target": "fs", "condition": null, "value": "mkdir(productosDir, recursive)" },
        { "type": "pseudo_step", "action": "assign", "target": "catalog", "condition": null, "value": "{ meta, categorias, productos }" },
        { "type": "pseudo_step", "action": "fetch", "target": "fs", "condition": null, "value": "writeFile(catalogPath, JSON.stringify(catalog))" }
      ],
      "throws": []
    },
    {
      "type": "method",
      "name": "handleUpdateProducto",
      "visibility": "public",
      "params": [
        { "name": "data", "type_expression": "{ project_id, id, ...cambios }", "required": true, "default": null }
      ],
      "returns": "{ status, data } | { status, error }",
      "pseudo_code": [
        { "type": "pseudo_step", "action": "assign", "target": "producto", "condition": null, "value": "{ ...existing, ...cambios }" },
        { "type": "pseudo_step", "action": "emit", "target": "producto.actualizado", "condition": null, "value": "{ project_id, producto_id, cambios }" },
        { "type": "pseudo_step", "action": "call", "target": "this.persistCatalog", "condition": null, "value": "project_id" },
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "T7: emit precede a persist; consumer leyendo disco ve estado pre-mutacion" }
      ],
      "throws": ["INVALID_INPUT", "RESOURCE_NOT_FOUND"]
    }
  ],

  "_auto_audit_resultado": {
    "pass": true,
    "detectores_ejecutados": [
      "string_fields_con_maxLength_o_pattern",
      "ausencia_de_conectores_discursivos",
      "ausencia_de_palabras_superiores_30_chars",
      "pseudo_step.action_dentro_del_enum_14",
      "ausencia_de_markdown_infiltrado",
      "ausencia_de_over_engineering_multinivel",
      "tendencias_enforcedTendencies_subset_T1_T8",
      "violations_con_evidencia_archivo_linea_y_snippet_real",
      "verificacion_terminos_contra_repo_via_Grep"
    ],
    "violaciones_detectadas_y_regeneradas": [
      {
        "violacion_inicial": "borrador inicial declaraba enforcedTendency T6 basandose en el cocinado",
        "deteccion": "lectura del codigo no encontro patron de timestamps inventados; el modulo usa new Date().toISOString() y crypto.randomUUID() que son canonicos",
        "regeneracion": "T6 removido de enforcedTendencies; observacion movida a _observaciones_para_ana"
      },
      {
        "violacion_inicial": "borrador inicial declaraba T4_catch_tragador",
        "deteccion": "Grep de catch tragador en index.js de productos: todos los catch capturan err y loggean error o warn; no hay catch vacio ni catch (_) {}",
        "regeneracion": "T4 no incluido en enforcedTendencies; el caso testigo de T4 en Fiel v0.1.0 apunta a modules/filesystem/index.js no a productos"
      }
    ]
  },

  "_observaciones_para_ana": [
    "VERIFICACION_T6: el cocinado pedia verificar si T6 aplicaba. Lectura del codigo confirma que NO aplica como violacion: timestamps via new Date().toISOString() y request_id via crypto.randomUUID() son patrones canonicos del repo, no invenciones. T6 captura el patron de inventar valores en campos no especificados por pseudo_code; en productos los timestamps SI estan especificados como parte del envelope canonico via _publicarEvento linea 124.",
    "VERIFICACION_T8: el termino nowISO citado en el contrato hermano _contrato-modulo-event-core-disciplina-via-notario.json no existe en modules/. Grep produce 0 hits en el dominio de modulos. Existe en blueprints y en doc pero no en codigo JS. El patron real del repo es new Date().toISOString() inline. No bloquea esta plasmation pero queda anotado para revision del contrato hermano.",
    "DRIFT_NUEVO_ENCONTRADO_EN_LECTURA: el manifest module.json declara subscribes a carta.actualizada con handler onCartaGenerada (no onCartaActualizada). La auditoria modulo-completo no reporta handlers porque parsea solo eventos; el desencuentro semantico nombre-de-evento vs nombre-de-handler es drift que ningun validator captura todavia.",
    "DRIFT_NUEVO_ENCONTRADO_EN_LECTURA: el module.json declara version 4.0.0 (linea 3) y el header del index.js documenta v2.3 (linea 2). Drift entre source-of-truth declarado y comentario humano. Auditoria reporta version 1.0.0 que es un tercer valor; los tres no coinciden.",
    "DRIFT_NUEVO_ENCONTRADO_EN_LECTURA: el property this.eventBus se llama eventBus en productos pero el contrato padre ModuloEventCore declara la property como bus. Hay desencuentro de nombre entre abstracto y concreto que conviene reconciliar en v0.2 del contrato padre o documentar como alias permitido.",
    "DRIFT_NUEVO_ENCONTRADO_EN_LECTURA: el manifest declara publishes catalogo.actualizado pero el codigo lo emite dos veces (lineas 173 y 938) con shapes distintos: una con campo source: 'disk_load' y otra sin. Es polisemia de evento bajo mismo nombre, anti-patron documentado en CLAUDE.md seccion garantias obligatorias en payloads.",
    "OBSERVACION_DE_ALCANCE: este documento NO declara contrato propio porque deriva del contrato hermano; es ConcreteClass testigo de drift. Si ana quiere que cada ConcreteClass tenga su contrato derivado (ej: pizzepos-productos.contract.json) hace falta decidir el shape del contrato derivado-de-clase en Fiel v0.2.0.",
    "OBSERVACION_DE_FORMA: el cocinado pedia verificar T6 'y posiblemente'. Auto-audit confirma que T6 no aplica y T4 tampoco. El subset confirmado de violations es exactamente {T1, T2, T3, T5, T7}, cinco tendencias. Las tres restantes T4, T6, T8 no tienen evidencia en este modulo."
  ]
}</result>
<usage><subagent_tokens>105651</subagent_tokens><tool_uses>36</tool_uses><duration_ms>283791</duration_ms></usage>
</task-notification>
