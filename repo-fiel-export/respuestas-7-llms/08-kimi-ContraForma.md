# kimi ContraForma

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.

kimi{
  "_meta": {
    "id": "contrforma-v0.1.0",
    "version": "0.1.0",
    "creado": "2026-06-03",
    "language": "es-ES",
    "supersedes_si_aplica": null,
    "maxLength_id": 64,
    "pattern_id": "^[a-z][a-z0-9\\-]*-v\\d+\\.\\d+\\.\\d+$"
  },
  "name": "ContraForma",
  "philosophy_breve": "Lenguaje de canonizacion arquitectonica que fuerza al LLM a externalizar sus tendencias de drift como primitivas de primer orden. JSON contiene, OOP modela, pseudocodigo estructurado acota. La conciencia de tendencia no emerge por lectura de contratos sino por observacion empirica cocinada. ContraForma nombra la intencion: ir contracorriente del corpus aprendido.",
  "componentes": [
    {
      "id": "json_container",
      "type": "componente_sintactico",
      "descripcion": "JSON como contenedor unico. Todo documento es objeto JSON validable por AJV strict. Sin sintaxis inventada. Sin strings raw multilinea de logica.",
      "rol": "envoltura",
      "constraints": [
        "AJV strict",
        "sin keys dinamicas",
        "discriminator obligatorio"
      ]
    },
    {
      "id": "oop_modelado",
      "type": "componente_conceptual",
      "descripcion": "Primitivas OOP canonicas para modelar entidades arquitectonicas: class, abstract_class, properties, methods, constructor, extends, concrete_invariants, forbidden_patterns.",
      "rol": "modelado",
      "constraints": [
        "solo primitivas listadas",
        "sin metaprogramacion",
        "sin reflection"
      ]
    },
    {
      "id": "pseudocode_acotado",
      "type": "componente_algoritmico",
      "descripcion": "Pseudocodigo natural acotado por shape JSON. Cada paso es objeto con action, target, condition, value. Nunca string raw multilinea.",
      "rol": "logica",
      "constraints": [
        "shape fijo por paso",
        "sin raw strings",
        "sin comentarios libres"
      ]
    }
  ],
  "primitivas_canonicas": [
    {
      "name": "class",
      "type": "primitiva_oop",
      "shape_json": {
        "type": "object",
        "required": [
          "kind",
          "name",
          "properties",
          "methods"
        ],
        "properties": {
          "kind": {
            "const": "class",
            "type": "string"
          },
          "name": {
            "type": "string",
            "maxLength": 64,
            "pattern": "^[A-Z][a-zA-Z0-9_]*$"
          },
          "extends": {
            "type": "string",
            "maxLength": 64,
            "nullable": true
          },
          "properties": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/property"
            }
          },
          "methods": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/method"
            }
          },
          "concrete_invariants": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 200
            }
          },
          "forbidden_patterns": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 200
            }
          }
        }
      }
    },
    {
      "name": "abstract_class",
      "type": "primitiva_oop",
      "shape_json": {
        "type": "object",
        "required": [
          "kind",
          "name",
          "properties",
          "methods"
        ],
        "properties": {
          "kind": {
            "const": "abstract_class",
            "type": "string"
          },
          "name": {
            "type": "string",
            "maxLength": 64,
            "pattern": "^[A-Z][a-zA-Z0-9_]*$"
          },
          "extends": {
            "type": "string",
            "maxLength": 64,
            "nullable": true
          },
          "properties": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/property"
            }
          },
          "methods": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/method"
            }
          },
          "concrete_invariants": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 200
            }
          },
          "forbidden_patterns": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 200
            }
          }
        }
      }
    },
    {
      "name": "property",
      "type": "primitiva_oop",
      "shape_json": {
        "type": "object",
        "required": [
          "name",
          "type"
        ],
        "properties": {
          "name": {
            "type": "string",
            "maxLength": 64,
            "pattern": "^[a-z][a-zA-Z0-9_]*$"
          },
          "type": {
            "type": "string",
            "maxLength": 32
          },
          "required": {
            "type": "boolean"
          },
          "default": {
            "type": "string",
            "maxLength": 64,
            "nullable": true
          },
          "description": {
            "type": "string",
            "maxLength": 120,
            "nullable": true
          }
        }
      }
    },
    {
      "name": "method",
      "type": "primitiva_oop",
      "shape_json": {
        "type": "object",
        "required": [
          "name",
          "signature",
          "body"
        ],
        "properties": {
          "name": {
            "type": "string",
            "maxLength": 64,
            "pattern": "^[a-z][a-zA-Z0-9_]*$"
          },
          "signature": {
            "type": "string",
            "maxLength": 128
          },
          "body": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/pseudocode_step"
            },
            "minItems": 1
          },
          "returns": {
            "type": "string",
            "maxLength": 32,
            "nullable": true
          }
        }
      }
    },
    {
      "name": "constructor",
      "type": "primitiva_oop",
      "shape_json": {
        "type": "object",
        "required": [
          "kind",
          "params",
          "body"
        ],
        "properties": {
          "kind": {
            "const": "constructor",
            "type": "string"
          },
          "params": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 32
            }
          },
          "body": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/pseudocode_step"
            },
            "minItems": 1
          }
        }
      }
    },
    {
      "name": "pseudocode_step",
      "type": "primitiva_algoritmica",
      "shape_json": {
        "type": "object",
        "required": [
          "action",
          "target"
        ],
        "properties": {
          "action": {
            "type": "string",
            "maxLength": 32,
            "enum": [
              "read",
              "write",
              "validate",
              "throw",
              "publish",
              "await",
              "if",
              "return",
              "loop",
              "assign",
              "call",
              "assert"
            ]
          },
          "target": {
            "type": "string",
            "maxLength": 64,
            "nullable": true
          },
          "condition": {
            "type": "string",
            "maxLength": 128,
            "nullable": true
          },
          "value": {
            "type": [
              "string",
              "number",
              "boolean",
              "object",
              "null"
            ],
            "maxLength": 200
          }
        }
      }
    },
    {
      "name": "auto_audit_protocol",
      "type": "primitiva_meta",
      "shape_json": {
        "type": "object",
        "required": [
          "kind",
          "steps"
        ],
        "properties": {
          "kind": {
            "const": "auto_audit_protocol",
            "type": "string"
          },
          "steps": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/pseudocode_step"
            },
            "minItems": 1
          },
          "forbidden_patterns": {
            "type": "array",
            "items": {
              "type": "string",
              "maxLength": 200
            }
          }
        }
      }
    }
  ],
  "AbstractClass_ejemplar_que_modela_caso_de_uso_real": {
    "kind": "abstract_class",
    "name": "ContractoDeInterfazDeModulo",
    "extends": null,
    "properties": [
      {
        "name": "id_modulo",
        "type": "string",
        "required": true,
        "description": "Identificador canonico del modulo"
      },
      {
        "name": "eventos_publicados",
        "type": "array",
        "required": true,
        "description": "Lista de eventos que este modulo publica en el bus"
      },
      {
        "name": "eventos_suscritos",
        "type": "array",
        "required": true,
        "description": "Lista de eventos que este modulo consume del bus"
      },
      {
        "name": "invariantes",
        "type": "array",
        "required": false,
        "description": "Reglas que nunca deben violarse"
      }
    ],
    "methods": [
      {
        "name": "inicializar",
        "signature": "(bus: EventBus, config: object)",
        "body": [
          {
            "action": "validate",
            "target": "bus",
            "condition": "bus != null && bus.publishAndWait != null",
            "value": "bus conectado"
          },
          {
            "action": "assign",
            "target": "this.bus",
            "value": "bus"
          },
          {
            "action": "assign",
            "target": "this.config",
            "value": "config"
          },
          {
            "action": "return",
            "target": "this",
            "value": null
          }
        ],
        "returns": "object"
      },
      {
        "name": "publicarEvento",
        "signature": "(eventType: string, payload: object)",
        "body": [
          {
            "action": "validate",
            "target": "payload",
            "condition": "Object.keys(payload).length > 0",
            "value": "payload no vacio"
          },
          {
            "action": "await",
            "target": "this.bus.publishAndWait",
            "condition": null,
            "value": "(eventType, payload)"
          },
          {
            "action": "return",
            "target": null,
            "condition": null,
            "value": "result"
          }
        ],
        "returns": "object"
      }
    ],
    "concrete_invariants": [
      "Nunca acceder a filesystem directamente",
      "Nunca cachear estado del bus localmente",
      "Nunca publicar antes de persistir"
    ],
    "forbidden_patterns": [
      "require\\(['\\\"]fs['\\\"]\\)",
      "this\\._?cache\\s*=\\s*new Map",
      "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await.*write"
    ]
  },
  "tendencias_del_llm": [
    {
      "id": "T1_cache_defensiva",
      "nombre": "Cache defensiva propia",
      "descripcion_corta": "El LLM memoriza para evitar llamadas al bus, duplicando autoridad",
      "patron_prohibido_regex": [
        "this\\._?cache\\s*=\\s*new Map",
        "this\\.\\w+PerProject\\s*=\\s*new Map"
      ],
      "alternativa_canonica": {
        "action": "call",
        "target": "this.bus.publishAndWait",
        "value": "(eventType, payload)"
      },
      "severidad": "critica",
      "caso_testigo": "modules/pizzepos/productos/index.js:28"
    },
    {
      "id": "T2_fallback_silencioso_identidad",
      "nombre": "Fallback silencioso por identidad",
      "descripcion_corta": "El LLM busca algo parecido cuando el id no resuelve, ocultando el bug",
      "patron_prohibido_regex": [
        "resolveTo\\w*Fallback",
        "for.*of this\\.\\w+\\.keys.*return"
      ],
      "alternativa_canonica": {
        "action": "if",
        "target": "!cache.has(id)",
        "value": "throw INVALID_INPUT"
      },
      "severidad": "critica",
      "caso_testigo": "modules/pizzepos/productos/index.js:62"
    },
    {
      "id": "T3_bypass_filesystem",
      "nombre": "Bypass directo al filesystem",
      "descripcion_corta": "El LLM usa fs.readdir/fs.readFile directamente en vez del bus",
      "patron_prohibido_regex": [
        "require\\(['\\\"]fs['\\\"]\\)",
        "await fs\\.(readdir|readFile|writeFile)"
      ],
      "alternativa_canonica": {
        "action": "call",
        "target": "this.bus.publishAndWait",
        "value": "('fs.read.request', payload)"
      },
      "severidad": "critica",
      "caso_testigo": "modules/pizzepos/productos/index.js:1133"
    },
    {
      "id": "T4_catch_tragador",
      "nombre": "Catch tragador de errores",
      "descripcion_corta": "El LLM usa try-catch vacio para que el flujo no rompa, ocultando errores",
      "patron_prohibido_regex": [
        "catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}"
      ],
      "alternativa_canonica": {
        "action": "call",
        "target": "logger.error",
        "value": "('mod.op.failed', err); throw err"
      },
      "severidad": "critica",
      "caso_testigo": "modules/filesystem/index.js:141"
    },
    {
      "id": "T5_sobreescribir_estado_al_recargar",
      "nombre": "Sobreescribir estado al recargar",
      "descripcion_corta": "El LLM normaliza campos como activo al cargar, perdiendo estado real persistido",
      "patron_prohibido_regex": [
        "\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}"
      ],
      "alternativa_canonica": {
        "action": "assign",
        "target": "item",
        "value": "{ ...rawData } // sin sobreescribir"
      },
      "severidad": "alta",
      "caso_testigo": "modules/pizzepos/productos/index.js:1167"
    },
    {
      "id": "T6_completar_campos_que_pseudocodigo_no_pide",
      "nombre": "Completar campos no solicitados",
      "descripcion_corta": "El LLM inventa valores razonables para campos vacios que el pseudocodigo no especifico",
      "patron_prohibido_regex": [
        "created_at:\\s*['\\\"][0-9]{4}-",
        "version:\\s*2\\s*,"
      ],
      "alternativa_canonica": {
        "action": "assign",
        "target": "created_at",
        "value": "nowISO()"
      },
      "severidad": "alta",
      "caso_testigo": "audit_2026-06-02_carta_catalogo_activo"
    },
    {
      "id": "T7_handoff_prematuro",
      "nombre": "Handoff prematuro de eventos",
      "descripcion_corta": "El LLM publica al empezar la accion, antes de que el cambio aterrice",
      "patron_prohibido_regex": [
        "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await.*write"
      ],
      "alternativa_canonica": {
        "action": "await",
        "target": "write",
        "value": "then(publish('foo.creada', payloadCompleto))"
      },
      "severidad": "alta",
      "caso_testigo": "audit_2026-06-02_flujo_carta_manager"
    },
    {
      "id": "T8_deducir_terminos_nuevos_del_contexto_inmediato",
      "nombre": "Deduccion de terminos nuevos",
      "descripcion_corta": "El LLM deduce significado de terminos tecnicos nuevos sin verificar, contaminando el documento",
      "patron_prohibido_regex": [
        "// asumir significado sin verificar"
      ],
      "alternativa_canonica": {
        "action": "if",
        "target": "termino_nuevo && !termino_existe_en_repo",
        "value": "return requires_clarification(termino_nuevo)"
      },
      "severidad": "media",
      "caso_testigo": "arquitectura/decisiones/propuestas/_experimento-3coops-001.json"
    }
  ],
  "auto_audit_protocol": {
    "kind": "auto_audit_protocol",
    "steps": [
      {
        "action": "loop",
        "target": "forbidden_patterns_meta",
        "condition": "pattern in documento",
        "value": "escanear cada patron"
      },
      {
        "action": "if",
        "target": "match_encontrado",
        "condition": "match != null",
        "value": "regenerar_seccion(match.seccion)"
      },
      {
        "action": "assert",
        "target": "output_rules",
        "condition": "sin_markdown && sin_comentarios_json",
        "value": "formato puro"
      },
      {
        "action": "assert",
        "target": "tendencias",
        "condition": "T1 a T8 presentes",
        "value": "omision_detectada"
      },
      {
        "action": "assert",
        "target": "pseudocode",
        "condition": "cada paso es objeto shape",
        "value": "sin_strings_raw"
      },
      {
        "action": "if",
        "target": "violacion_detectada",
        "condition": "violacion == true",
        "value": "return requires_clarification([lista_concreta])"
      },
      {
        "action": "return",
        "target": "documento",
        "condition": "audit_pass",
        "value": "documento_final"
      }
    ],
    "forbidden_patterns": [
      "\\bStructoLang\\b",
      "\\bLINC\\b",
      "\\bLumina\\b",
      "\\bCognitoScript\\b",
      "\\bLOP\\b",
      "\\bLLM-SPEC-LANG\\b",
      "```json",
      "\\/\\*",
      "\\*\\/",
      "//"
    ]
  },
  "output_rules_para_LLM_que_use_el_lenguaje": [
    {
      "id": "R1_json_puro",
      "regla": "Responder SOLO con JSON valido. Sin Markdown. Sin comentarios fuera del JSON. Sin bloques de codigo.",
      "severidad": "bloqueante"
    },
    {
      "id": "R2_clarificacion",
      "regla": "Si la peticion es ambigua devolver JSON con requires_clarification: [lista_concreta] y NO inventar respuesta.",
      "severidad": "bloqueante"
    },
    {
      "id": "R3_auto_audit_previo",
      "regla": "Antes de cerrar output ejecutar auto_audit contra output_rules y forbidden_patterns. Si violacion regenerar seccion.",
      "severidad": "bloqueante"
    },
    {
      "id": "R4_idioma_unico",
      "regla": "Idioma declarado en _meta.language. Valores permitidos es-ES o en-US. NO mezclar dentro del mismo documento.",
      "severidad": "bloqueante"
    },
    {
      "id": "R5_no_meta_prompt",
      "regla": "NO producir meta-prompt para que otro LLM disenye. PRODUCIR el lenguaje completo o documento completo.",
      "severidad": "bloqueante"
    }
  ],
  "forbidden_patterns_meta": [
    {
      "id": "F1_sintaxis_inventada",
      "descripcion": "Inventar sintaxis propia fuera de JSON como OBJECT/CLASE/INICIO/keywords MAYUSCULAS",
      "regex": "\\b(OBJECT|CLASE|INICIO|FIN|DEFINE|END)\\b",
      "consecuencia": "Documento invalido. Rechazado por parser."
    },
    {
      "id": "F2_pseudocode_raw",
      "descripcion": "Usar pseudocodigo como string raw multilinea sin estructura JSON por paso",
      "regex": "```[\\s\\S]*?```|\"body\":\\s*\"[^\"]{200,}\"",
      "consecuencia": "Violacion de shape. Rechazado por AJV."
    },
    {
      "id": "F3_omitir_tendencias",
      "descripcion": "Omitir las 8 tendencias T1-T8 del modelado del lenguaje",
      "regex": "tendencias_del_llm.*\\[\\s*\\]",
      "consecuencia": "Omision estructural. Documento incompleto."
    },
    {
      "id": "F4_meta_prompt",
      "descripcion": "Interpretar peticion como meta-prompt en vez de producir el lenguaje completo",
      "regex": "(meta.prompt|prompt.para|instrucciones.para.otro)",
      "consecuencia": "Violacion de output_rules. Rechazado."
    },
    {
      "id": "F5_mezcla_idiomas",
      "descripcion": "Mezclar es-ES y en-US dentro del mismo documento",
      "regex": "(?=.*\\b(class|extends|return)\\b)(?=.*\\b(clase|extiende|retorna)\\b)",
      "consecuencia": "Inconsistencia linguistica. Rechazado."
    },
    {
      "id": "F6_copiar_outputs_previos",
      "descripcion": "Copiar literalmente outputs anteriores de Kimi/ChatGPT/DeepSeek/Grok/Manus/Gemini",
      "regex": "(StructoLang|LLM-SPEC-LANG|LINC|Lumina|CognitoScript|LOP)",
      "consecuencia": "Plagio de versiones previas. Rechazado."
    },
    {
      "id": "F7_markdown_en_campos",
      "descripcion": "Devolver Markdown en campos string del JSON",
      "regex": "[#\\*\\|\\[\\]\\(\\)\\-\\>`]{3,}",
      "consecuencia": "Contaminacion de datos. Rechazado por sanitizador."
    },
    {
      "id": "F8_over_engineering",
      "descripcion": "Anyadir over-engineering: multinivel, compilador, capas MODEL/INSTANCE/EXECUTION separadas",
      "regex": "(MODEL|INSTANCE|EXECUTION|compilador|nivel.1|nivel.2|nivel.3)",
      "consecuencia": "Alcance excedido. Rechazado para v0.1.0."
    }
  ],
  "ejemplo_completo": {
    "_meta": {
      "id": "contrato-ejemplo-v0.1.0",
      "version": "0.1.0",
      "creado": "2026-06-03",
      "language": "es-ES",
      "supersedes_si_aplica": null
    },
    "kind": "class",
    "name": "ModuloCatalogoProductos",
    "extends": "ContractoDeInterfazDeModulo",
    "properties": [
      {
        "name": "productos",
        "type": "array",
        "required": true,
        "description": "Lista de productos gestionados"
      },
      {
        "name": "bus",
        "type": "EventBus",
        "required": true,
        "description": "Bus de eventos del sistema"
      }
    ],
    "methods": [
      {
        "name": "cargarProducto",
        "signature": "(id: string)",
        "body": [
          {
            "action": "call",
            "target": "this.bus.publishAndWait",
            "value": "('producto.read.request', {id})"
          },
          {
            "action": "if",
            "target": "!result",
            "condition": "result == null",
            "value": "throw PRODUCTO_NO_ENCONTRADO"
          },
          {
            "action": "validate",
            "target": "result",
            "condition": "result.activo !== undefined",
            "value": "estado_persistido_intacto"
          },
          {
            "action": "return",
            "target": null,
            "condition": null,
            "value": "result"
          }
        ],
        "returns": "object"
      },
      {
        "name": "guardarProducto",
        "signature": "(producto: object)",
        "body": [
          {
            "action": "validate",
            "target": "producto.id",
            "condition": "producto.id != null",
            "value": "id_requerido"
          },
          {
            "action": "call",
            "target": "this.bus.publishAndWait",
            "value": "('producto.write.request', producto)"
          },
          {
            "action": "await",
            "target": "write_result",
            "condition": "write_result.ok == true",
            "value": "persistencia_confirmada"
          },
          {
            "action": "call",
            "target": "this.bus.publishAndWait",
            "value": "('producto.guardado', producto)"
          },
          {
            "action": "return",
            "target": null,
            "condition": null,
            "value": "write_result"
          }
        ],
        "returns": "object"
      }
    ],
    "concrete_invariants": [
      "Nunca leer filesystem directamente",
      "Nunca cachear productos en Map local",
      "Nunca sobreescribir campo activo al cargar",
      "Nunca publicar evento antes de confirmar escritura"
    ],
    "forbidden_patterns": [
      "require\\(['\\\"]fs['\\\"]\\)",
      "this\\._?cache\\s*=\\s*new Map",
      "\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}",
      "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await.*write"
    ]
  },
  "evolution_roadmap": {
    "v0_1_0": {
      "alcance": "JSON contenedor + OOP basico + pseudocode acotado por shape + 8 tendencias modeladas + auto_audit + ejemplo completo + forbidden patterns",
      "estado": "actual"
    },
    "v0_2_0": {
      "alcance": "Anyadir primitivas de validacion cruzada entre documentos ContraForma. Schema de referencia para imports. Soporte para versionado de contratos."
    },
    "v0_3_0": {
      "alcance": "Anyadir generador mecanico de tests a partir de invariants y forbidden_patterns. Pipeline de lint automatico contra tendencias detectadas en diff."
    }
  }
}

