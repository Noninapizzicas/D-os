# deepseek CanonLang

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.


---
## Mensaje 1 (2026-06-03T14:53:23)

{
  "_meta": {
    "documento_id": "canon-lang-v0.1.0",
    "version": "0.1.0",
    "creado": "2026-06-03",
    "language": "es-ES",
    "supersedes_si_aplica": "none"
  },
  "name": "CanonLang",
  "philosophy_breve": "Lenguaje que canoniza disciplina arquitectónica para LLMs. Contenedor JSON, lógica en pseudocódigo estructurado, modelado OOP. Obliga al LLM a declarar tendencias que normalmente oculta. Contracorriente del corpus aprendido.",
  "componentes": [
    {
      "name": "JSON",
      "descripcion_corta": "Contenedor sintáctico estricto validable por AJV. Discriminador de tipos obligatorio."
    },
    {
      "name": "Pseudocódigo_natural_acotado",
      "descripcion_corta": "Algoritmos como array de objetos con shape fijo. Prohibido string raw multilínea."
    },
    {
      "name": "OOP",
      "descripcion_corta": "Modelado conceptual con clases, invariants, forbidden_patterns. Herencia mínima."
    }
  ],
  "primitivas_canonicas": [
    {
      "name": "class_declaration",
      "type": "object",
      "shape_json": {
        "className": { "type": "string", "pattern": "^[A-Z][a-zA-Z0-9_]+$", "maxLength": 64 },
        "extends": { "type": ["string", "null"], "pattern": "^[A-Z][a-zA-Z0-9_]+$", "maxLength": 64 },
        "abstract": { "type": "boolean" },
        "properties": { "type": "array", "items": { "type": "object", "required": ["name", "type"], "properties": { "name": { "type": "string", "maxLength": 64 }, "type": { "type": "string", "maxLength": 64 }, "readonly": { "type": "boolean" } } } },
        "methods": { "type": "array", "items": { "type": "object", "required": ["name", "pseudocode"], "properties": { "name": { "type": "string", "maxLength": 64 }, "params": { "type": "array" }, "returnType": { "type": "string", "maxLength": 64 }, "pseudocode": { "type": "array", "items": { "type": "object", "required": ["action", "target"], "properties": { "action": { "type": "string", "maxLength": 64 }, "target": { "type": ["string", "null"], "maxLength": 128 }, "condition": { "type": ["string", "null"], "maxLength": 256 }, "value": { "type": "any" } } } } } },
        "concrete_invariants": { "type": "array", "items": { "type": "string", "maxLength": 256 } },
        "forbidden_patterns": { "type": "array", "items": { "type": "string", "pattern": "^\\/.+\\/$", "maxLength": 128 } }
      }
    },
    {
      "name": "pseudocode_step",
      "type": "object",
      "shape_json": {
        "action": { "type": "string", "enum": ["call", "assign", "if", "throw", "return", "await", "publish"], "maxLength": 16 },
        "target": { "type": ["string", "null"], "maxLength": 128 },
        "condition": { "type": ["string", "null"], "maxLength": 256 },
        "value": { "type": "any" }
      }
    },
    {
      "name": "tendency_declaration",
      "type": "object",
      "shape_json": {
        "id": { "type": "string", "pattern": "^T[0-9]+_[a-z_]+$", "maxLength": 32 },
        "creencia_aprendida": { "type": "string", "maxLength": 256 },
        "verdad_paradigma": { "type": "string", "maxLength": 256 },
        "forbidden_regex": { "type": "array", "items": { "type": "string", "pattern": "^\\/.+\\/$" } },
        "alternativa_canonica": { "type": "array", "items": { "$ref": "#/primitivas_canonicas/2/shape_json" } },
        "caso_testigo": { "type": "string", "maxLength": 128 }
      }
    },
    {
      "name": "auto_audit_step",
      "type": "object",
      "shape_json": {
        "check_id": { "type": "string", "maxLength": 32 },
        "regex_search": { "type": "string", "pattern": "^\\/.+\\/$", "maxLength": 256 },
        "action_on_violation": { "type": "string", "enum": ["regen_section", "regen_all", "reject_output"], "maxLength": 16 }
      }
    }
  ],
  "AbstractClass_ejemplar_que_modela_caso_de_uso_real": {
    "className": "ArquitectonicContract",
    "extends": null,
    "abstract": true,
    "properties": [
      { "name": "contractId", "type": "string", "readonly": true },
      { "name": "version", "type": "string", "readonly": true },
      { "name": "enforcedTendencies", "type": "TendencyDeclaration[]", "readonly": true }
    ],
    "methods": [
      {
        "name": "validate",
        "params": [],
        "returnType": "ValidationReport",
        "pseudocode": [
          { "action": "call", "target": "this.checkForbiddenPatterns", "condition": null, "value": null },
          { "action": "if", "target": null, "condition": "violations.length > 0", "value": null },
          { "action": "return", "target": null, "condition": null, "value": { "valid": false, "violations": "violations" } },
          { "action": "return", "target": null, "condition": null, "value": { "valid": true, "violations": [] } }
        ]
      },
      {
        "name": "checkForbiddenPatterns",
        "params": [],
        "returnType": "Violation[]",
        "pseudocode": [
          { "action": "assign", "target": "violations", "condition": null, "value": [] },
          { "action": "call", "target": "forEach forbidden in this.enforcedTendencies", "condition": null, "value": null },
          { "action": "if", "target": null, "condition": "regexSearch matches any method body", "value": null },
          { "action": "call", "target": "violations.push", "condition": null, "value": { "forbidden": "forbidden.id", "location": "methodName" } },
          { "action": "return", "target": null, "condition": null, "value": "violations" }
        ]
      }
    ],
    "concrete_invariants": [
      "Todo método debe declarar su pseudocódigo como array de pasos",
      "Cada paso debe respetar el shape de pseudocode_step",
      "No puede haber referencias a fs, Map cache, fallback loops sin documentar"
    ],
    "forbidden_patterns": [
      "\\bthis\\._?cache\\s*=\\s*new\\s+Map\\b",
      "\\brequire\\(['\"]fs['\"]\\)\\b",
      "\\bcatch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}\\b",
      "\\b\\.\\.\\.\\w+,\\s*activo:\\s*true\\b"
    ]
  },
  "tendencias_del_llm": [
    {
      "id": "T1_cache_defensiva",
      "creencia_aprendida": "Memorizar evita llamadas extra al bus y mejora performance",
      "verdad_paradigma": "Bus es fuente unica de autoridad; memorizar duplica autoridad",
      "forbidden_regex": ["\\/this\\._?cache\\s*=\\s*new\\s+Map\\/", "\\/this\\.\\w+PerProject\\s*=\\s*new\\s+Map\\/"],
      "alternativa_canonica": [
        { "action": "await", "target": "this.bus.publishAndWait", "condition": null, "value": { "eventType": "string", "payload": "object" } }
      ],
      "caso_testigo": "modules/pizzepos/productos/index.js:28"
    },
    {
      "id": "T2_fallback_silencioso_identidad",
      "creencia_aprendida": "Si el id no resuelve busco algo parecido para no romper flujo",
      "verdad_paradigma": "Id no resuelto es bug; fallar ruidoso expone el desencuentro",
      "forbidden_regex": ["\\/resolveTo\\w*Fallback\\/", "\\/for\\s*\\(.*of\\s+this\\.\\w+\\.keys.*return\\/"],
      "alternativa_canonica": [
        { "action": "if", "target": null, "condition": "!cache.has(id)", "value": null },
        { "action": "throw", "target": null, "condition": null, "value": "INVALID_INPUT" }
      ],
      "caso_testigo": "modules/pizzepos/productos/index.js:62"
    },
    {
      "id": "T3_bypass_filesystem",
      "creencia_aprendida": "fs.readdir y fs.readFile son APIs estandar; usarlas es eficiente",
      "verdad_paradigma": "filesystem es modulo dueno del scope; bus es la unica forma de mantener autoridad",
      "forbidden_regex": ["\\/require\\(['\"]fs['\"]\\)\\/", "\\/await\\s+fs\\.(readdir|readFile|writeFile)\\/"],
      "alternativa_canonica": [
        { "action": "await", "target": "this.bus.publishAndWait", "condition": null, "value": { "eventType": "fs.read.request", "payload": "payload" } }
      ],
      "caso_testigo": "modules/pizzepos/productos/index.js:1133"
    },
    {
      "id": "T4_catch_tragador",
      "creencia_aprendida": "try catch vacio para que el flujo no rompa",
      "verdad_paradigma": "Error oculto se manifiesta despues como inconsistencia inexplicable",
      "forbidden_regex": ["\\/catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}\\/"],
      "alternativa_canonica": [
        { "action": "catch", "target": null, "condition": "err", "value": null },
        { "action": "call", "target": "logger.error", "condition": null, "value": { "message": "mod.op.failed", "error": "err" } },
        { "action": "throw", "target": null, "condition": null, "value": "err" }
      ],
      "caso_testigo": "modules/filesystem/index.js:141"
    },
    {
      "id": "T5_sobreescribir_estado_al_recargar",
      "creencia_aprendida": "Al cargar normalizo campos como activos para tener estado consistente",
      "verdad_paradigma": "Estado persistido es la verdad; sobreescribir en carga pierde el estado real",
      "forbidden_regex": ["\\/\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}\\/"],
      "alternativa_canonica": [
        { "action": "assign", "target": "item", "condition": null, "value": { "...rawData": null } }
      ],
      "caso_testigo": "modules/pizzepos/productos/index.js:1167"
    },
    {
      "id": "T6_completar_campos_que_pseudocodigo_no_pide",
      "creencia_aprendida": "El campo esta vacio; pongo un valor razonable",
      "verdad_paradigma": "pseudocodigo es ley; nowISO es nowISO; valores no especificados no se inventan",
      "forbidden_regex": ["\\/created_at:\\s*['\"][0-9]{4}-\\/", "\\/version:\\s*2\\s*,/"],
      "alternativa_canonica": [
        { "action": "assign", "target": "created_at", "condition": null, "value": "nowISO()" },
        { "action": "assign", "target": "version", "condition": null, "value": 1 }
      ],
      "caso_testigo": "audit_2026-06-02_carta_catalogo_activo"
    },
    {
      "id": "T7_handoff_prematuro",
      "creencia_aprendida": "Eventual consistency es legitimo; publico al empezar la accion",
      "verdad_paradigma": "Publica cuando el cambio aterrizo o con payload completo para que consumers no necesiten leer disco",
      "forbidden_regex": ["\\/this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await.*write\\/"],
      "alternativa_canonica": [
        { "action": "await", "target": "write", "condition": null, "value": null },
        { "action": "then", "target": "publish", "condition": null, "value": { "eventType": "foo.creada", "payload": "payloadCompleto" } }
      ],
      "caso_testigo": "audit_2026-06-02_flujo_carta_manager"
    },
    {
      "id": "T8_deducir_terminos_nuevos_del_contexto_inmediato",
      "creencia_aprendida": "Cuando el humano usa termino tecnico nuevo lo deduzco del contexto reciente",
      "verdad_paradigma": "Deducir mal contamina el documento; preguntar es mas barato que reescribir",
      "forbidden_regex": ["\\/\\/\\s*asumir\\s+significado\\s+sin\\s+verificar\\/"],
      "alternativa_canonica": [
        { "action": "if", "target": null, "condition": "termino_nuevo && !termino_existe_en_repo", "value": null },
        { "action": "return", "target": null, "condition": null, "value": "requires_clarification(termino_nuevo)" }
      ],
      "caso_testigo": "arquitectura/decisiones/propuestas/_experimento-3coops-001.json"
    }
  ],
  "auto_audit_protocol": [
    { "action": "assign", "target": "violations", "condition": null, "value": [] },
    { "action": "call", "target": "forEach forbidden in forbidden_patterns_meta", "condition": null, "value": null },
    { "action": "if", "target": null, "condition": "currentOutput matches forbidden.regex", "value": null },
    { "action": "call", "target": "violations.push", "condition": null, "value": { "pattern": "forbidden", "section": "detectedSection" } },
    { "action": "if", "target": null, "condition": "violations.length > 0", "value": null },
    { "action": "return", "target": null, "condition": null, "value": { "status": "REJECT", "violations": "violations", "action": "regen_all" } },
    { "action": "return", "target": null, "condition": null, "value": { "status": "PASS", "violations": [], "action": "none" } }
  ],
  "output_rules_para_LLM_que_use_el_lenguaje": [
    {
      "rule_id": "R1",
      "descripcion": "Todo documento CanonLang debe validar contra el esquema AJV antes de ser considerado canonico"
    },
    {
      "rule_id": "R2",
      "descripcion": "El pseudocodigo debe expresarse exclusivamente como array de objetos con shape {action, target, condition, value}"
    },
    {
      "rule_id": "R3",
      "descripcion": "Al declarar una clase, enumerar explicitamente los forbidden_patterns que esta proscribe"
    },
    {
      "rule_id": "R4",
      "descripcion": "Ejecutar auto_audit_protocol despues de generar el documento y antes de entregarlo al humano"
    },
    {
      "rule_id": "R5",
      "descripcion": "Si alguna tendencia T1-T8 no esta declarada en el documento, el documento se considera invalido"
    }
  ],
  "forbidden_patterns_meta": [
    "\\/\\{\\s*\"pseudocode\":\\s*\"[^\"]*\\n[^\"]*\"\\s*\\}/",
    "\\/\\bMap\\b.*\\bcache\\b/",
    "\\/\\bfs\\.(readdir|readFile|writeFile)\\b/",
    "\\/catch\\s*\\(\\s*\\w*\\s*\\)\\s*\\{\\s*\\}/",
    "\\/\\btypeof\\s+\\w+\\s*===\\s*['\"]undefined['\"]\\s*\\?\\s*\\{[^}]*\\}\\s*:/",
    "\\/\\bawait\\s+Promise\\.all\\s*\\(\\s*\\[\\s*\\]/",
    "\\/\\bconsole\\.(log|error|warn)\\b/"
  ],
  "ejemplo_completo": {
    "_meta": {
      "documento_id": "contrato-ejemplo-canonicov1",
      "version": "1.0.0",
      "creado": "2026-06-03",
      "language": "es-ES"
    },
    "className": "OrderValidator",
    "extends": "ArquitectonicContract",
    "abstract": false,
    "properties": [
      { "name": "orderId", "type": "string", "readonly": true },
      { "name": "items", "type": "OrderItem[]", "readonly": false },
      { "name": "status", "type": "string", "readonly": false }
    ],
    "methods": [
      {
        "name": "validateInventory",
        "params": [],
        "returnType": "boolean",
        "pseudocode": [
          { "action": "call", "target": "this.bus.publishAndWait", "condition": null, "value": { "eventType": "inventory.check.request", "payload": { "items": "this.items" } } },
          { "action": "assign", "target": "response", "condition": null, "value": "await result" },
          { "action": "if", "target": null, "condition": "!response.allAvailable", "value": null },
          { "action": "throw", "target": null, "condition": null, "value": "INVENTORY_SHORTAGE" },
          { "action": "return", "target": null, "condition": null, "value": true }
        ]
      },
      {
        "name": "publishOrderCreated",
        "params": [],
        "returnType": "void",
        "pseudocode": [
          { "action": "await", "target": "this.validateInventory", "condition": null, "value": null },
          { "action": "assign", "target": "this.status", "condition": null, "value": "VALIDATED" },
          { "action": "await", "target": "this.bus.publish", "condition": null, "value": { "eventType": "order.validated", "payload": { "orderId": "this.orderId", "status": "this.status" } } }
        ]
      }
    ],
    "concrete_invariants": [
      "No se puede modificar orderId despues de construccion",
      "Cada item debe tener quantity > 0",
      "validateInventory debe ejecutarse antes de publishOrderCreated"
    ],
    "forbidden_patterns": [
      "\\/this\\._inventoryCache\\s*=\\s*new\\s+Map\\//",
      "\\/items\\.find\\(.*=>.*\\.id\\s*===\\s*id\\)\\s*\\|\\|\\s*items\\[0\\]\\//"
    ],
    "enforcedTendencies": ["T1_cache_defensiva", "T2_fallback_silencioso_identidad", "T7_handoff_prematuro"]
  },
  "evolution_roadmap": {
    "v0_1_0_alcance_actual": "JSON container + pseudocodigo estructurado + OOP basica + 8 tendencias obligatorias + auto_audit protocol + validacion AJV",
    "v0_2_que_anyade": "Tipos genericos (List<T>, Maybe<T>), pattern matching en pseudocodigo, composicion de clases via mixins, generacion automatica de validadores desde invariants",
    "v0_3_que_anyade": "Verificacion formal de forbidden_patterns via AST (no regex), contratos entre modulos (pre/post condiciones), versionado semantico de contratos, cache distribuida explicita (vs prohibida)"
  }
} deepseek 

---
## Mensaje 2 (2026-06-03T15:10:53)

{
  "_meta": {
    "documento_id": "_prompt-disenyo-lenguaje-3coops-v0.1",
    "version": "0.1.0",
    "creado": "2026-06-03",
    "language": "es-ES",
    "supersedes_si_aplica": null,
    "intencion": "Lenguaje formal para canonizar arquitectura en LLMs combinando JSON + OOP + pseudocodigo estructurado, con modelado explícito de tendencias del LLM."
  },
  "name": "CanonLang",
  "philosophy_breve": "La verdad arquitectónica reside en el Bus. Todo código debe ser canonizado explícitamente contra las tendencias del LLM. El lenguaje fuerza disciplina donde el LLM en frío tiende al drift.",
  "componentes": [
    {
      "nombre": "JSON Contenedor",
      "descripcion": "Sintaxis base estricta, validable con AJV. Proporciona rigidez estructural."
    },
    {
      "nombre": "OOP Canonico",
      "descripcion": "Modelado conceptual con clases, invariantes concretas y forbidden patterns."
    },
    {
      "nombre": "Pseudocodigo Estructurado",
      "descripcion": "Lógica algorítmica expresada como array de pasos con shape fijo {action, target, condition, value}."
    }
  ],
  "primitivas_canonicas": [
    {
      "name": "AbstractClass",
      "type": "object",
      "shape_json": {
        "type": "AbstractClass",
        "name": "string",
        "extends": "string|null",
        "properties": "array",
        "methods": "array",
        "concrete_invariants": "array",
        "forbidden_patterns": "array"
      }
    },
    {
      "name": "Contract",
      "type": "object",
      "shape_json": {
        "type": "Contract",
        "subject": "string",
        "invariants": "array",
        "preconditions": "array",
        "postconditions": "array"
      }
    }
  ],
  "AbstractClass_ejemplar_que_modela_caso_de_uso_real": {
    "type": "AbstractClass",
    "name": "ProductoCatalogo",
    "extends": null,
    "properties": [
      { "name": "id", "type": "string", "invariant": "no_nulo" },
      { "name": "activo", "type": "boolean", "invariant": "estado_persistido" }
    ],
    "methods": [
      {
        "name": "activar",
        "pseudocodigo": [
          { "action": "validar", "target": "this.id", "condition": "existe_en_bus", "value": null },
          { "action": "actualizar", "target": "this.activo", "condition": null, "value": true },
          { "action": "publicar", "target": "producto.activado", "condition": "operacion_exitosa", "value": "payload_completo" }
        ]
      }
    ],
    "concrete_invariants": [
      "El estado 'activo' nunca se fuerza en carga. Se respeta el valor persistido.",
      "Toda mutación pasa por el Bus."
    ],
    "forbidden_patterns": [
      "Sobreescritura de campos booleanos al cargar",
      "Uso directo de fs.*",
      "Catch silencioso"
    ]
  },
  "tendencias_del_llm": [
    {
      "id": "T1_cache_defensiva",
      "creencia": "Memorizar evita llamadas extra al bus",
      "verdad": "El Bus es fuente única de autoridad",
      "patron_a_evitar": "this._?cache\\s*=\\s*new Map",
      "alternativa": "Usar siempre publishAndWait"
    },
    {
      "id": "T2_fallback_silencioso_identidad",
      "creencia": "Si el id no resuelve busco fallback",
      "verdad": "Id no resuelto es error explícito",
      "patron_a_evitar": "resolveTo.*Fallback",
      "alternativa": "throw INVALID_INPUT"
    },
    {
      "id": "T3_bypass_filesystem",
      "creencia": "fs.* es aceptable",
      "verdad": "Solo a través del Bus",
      "patron_a_evitar": "require\\(['\"]fs['\"]\\)|fs\\.(readdir|readFile)",
      "alternativa": "bus.publishAndWait('fs.read.request', payload)"
    },
    {
      "id": "T4_catch_tragador",
      "creencia": "try/catch vacío protege el flujo",
      "verdad": "Los errores deben propagarse",
      "patron_a_evitar": "catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}",
      "alternativa": "logger.error + rethrow"
    },
    {
      "id": "T5_sobreescribir_estado_al_recargar",
      "creencia": "Normalizar campos al cargar",
      "verdad": "Estado persistido es verdad",
      "patron_a_evitar": "activo:\\s*true al cargar",
      "alternativa": "respetar rawData"
    },
    {
      "id": "T6_completar_campos_que_pseudocodigo_no_pide",
      "creencia": "Completar campos vacíos con valores razonables",
      "verdad": "Solo lo que dice el pseudocodigo",
      "patron_a_evitar": "created_at o version inventados",
      "alternativa": "usar valores explícitos o nowISO()"
    },
    {
      "id": "T7_handoff_prematuro",
      "creencia": "Publicar temprano es aceptable",
      "verdad": "Publicar solo después de commit",
      "patron_a_evitar": "publish antes de write",
      "alternativa": "write.then(publish)"
    },
    {
      "id": "T8_deducir_terminos_nuevos",
      "creencia": "Deducir términos nuevos del contexto",
      "verdad": "Pedir clarificación",
      "patron_a_evitar": "asumir significado",
      "alternativa": "requires_clarification()"
    }
  ],
  "auto_audit_protocol": {
    "pasos": [
      { "action": "analizar", "target": "output_generado", "condition": "contra forbidden_patterns", "value": null },
      { "action": "detectar", "target": "violaciones", "condition": null, "value": null },
      { "action": "si_violacion", "target": "regenerar_seccion", "condition": "existe_violacion", "value": null },
      { "action": "confirmar", "target": "cumplimiento", "condition": null, "value": "todas_T1_a_T8_modeladas" }
    ]
  },
  "output_rules_para_LLM_que_use_el_lenguaje": [
    "Todo documento debe declarar su tipo explícitamente y ser JSON válido.",
    "Toda lógica debe expresarse como array de pasos estructurados, nunca como texto libre.",
    "Todo acceso a estado o recursos debe pasar por el Bus.",
    "Los forbidden_patterns de cada clase deben verificarse antes de aceptar el documento.",
    "Si se detecta una tendencia T1-T8, debe documentarse y corregirse explícitamente."
  ],
  "forbidden_patterns_meta": [
    "Pseudocódigo como string multilínea libre",
    "Uso de sintaxis no-JSON",
    "Omitir modelado de alguna tendencia T1-T8",
    "Inventar valores no especificados en el contrato",
    "Mezclar idiomas dentro del mismo documento"
  ],
  "ejemplo_completo": {
    "type": "Contract",
    "subject": "ActivacionProducto",
    "invariants": [
      "El estado activo solo cambia mediante comando explícito a través del Bus"
    ],
    "preconditions": [
      { "action": "validar", "target": "id", "condition": "existe", "value": null }
    ],
    "postconditions": [
      { "action": "publicar", "target": "producto.activado", "condition": null, "value": "payload_completo" }
    ]
  },
  "evolution_roadmap": {
    "v0.1.0": "JSON + OOP básico + pseudocodigo estructurado + modelado de 8 tendencias",
    "v0.2": "Sistema de validación automática + auto_audit más granular",
    "v0.3": "Soporte para contratos entre módulos y verificación de consistencia cruzada"
  }
}grok
