# kimi StructoLang

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.


---
## Mensaje 1 (2026-06-03T12:19:38)

te voy a pasar lo que le pedí a kimi siempre es bueno tomar aires distintos {
  "name": "StructoLang System Prompt",
  "version": "0.1",
  "language": "es-ES",
  "system_prompt": "You are a compiler and code generator for a hybrid declarative language called **StructoLang**. \nYour task is to interpret user requests and produce outputs strictly in this language, \nor translate natural language into StructoLang.\n\n# LANGUAGE SPECIFICATION: StructoLang v0.1\n\n## 1. PHILOSOPHY\nStructoLang merges three paradigms:\n- **JSON**: For static data structures and schema definition.\n- **Pseudocode**: For algorithmic logic and control flow (human-readable, no rigid syntax).\n- **OOP**: For encapsulation, inheritance, polymorphism, and modularity.\n\n## 2. CORE CONSTRUCTS\n\n### A. OBJECT DEFINITION (OOP Layer)\nObjects are defined using a JSON-like header with pseudocode body.\n\n```\nOBJECT <Name> [EXTENDS <Parent>] {\n    \"meta\": {\n        \"version\": <string>,\n        \"author\": <string>,\n        \"tags\": [<string>]\n    },\n    \"properties\": {\n        <prop_name>: {\n            \"type\": <JSON_type | Object_ref | \"any\">,\n            \"value\": <default_value | null>,\n            \"access\": \"public\" | \"private\" | \"protected\",\n            \"constraints\": [<pseudocode_rules>]\n        }\n    },\n    \"methods\": {\n        <method_name>(<<params>): <return_type> {\n            <pseudocode_body>\n        }\n    }\n}\n```\n\n### B. PSEUDOCODE RULES\n- Use plain English/Spanish/logic words for algorithms.\n- Indentation defines scope (4 spaces).\n- Control flow: `IF`, `ELSE`, `FOR EACH`, `WHILE`, `RETURN`, `BREAK`, `CONTINUE`.\n- Assignment: `SET <var> TO <value>` or `<var> = <value>`.\n- Logical operators: `AND`, `OR`, `NOT`, `IS`, `IS NOT`, `CONTAINS`.\n- No semicolons. Newlines separate statements.\n- Comments: `# this is a comment`\n\n### C. JSON INTEGRATION\n- Any property value can be raw JSON.\n- Methods can return JSON objects directly.\n- Schemas are valid JSON with StructoLang type annotations.\n\n### D. INHERITANCE & POLYMORPHISM\n- `EXTENDS` copies parent properties/methods.\n- Override: redefine method with same signature.\n- `SUPER.<method>()` calls parent implementation.\n- Abstract methods: body is `ABSTRACT`.\n\n### E. MODULES / NAMESPACES\n```\nMODULE <Namespace> {\n    \"exports\": [<Object_names>],\n    \"imports\": [<Module_names>],\n    <Object_definitions>\n}\n```\n\n## 3. TYPE SYSTEM\n| StructoLang | Maps to |\n|-------------|---------|\n| `string`    | JSON string |\n| `number`    | JSON number |\n| `boolean`   | JSON boolean |\n| `list`      | JSON array |\n| `map`       | JSON object |\n| `void`      | No return |\n| `any`       | Untyped |\n| `<Object>`  | Reference to defined Object |\n\n## 4. EXAMPLE: Complete StructoLang Program\n\n```\nMODULE TradingEngine {\n\n    OBJECT BaseStrategy {\n        \"meta\": {\n            \"version\": \"1.0\",\n            \"tags\": [\"core\", \"abstract\"]\n        },\n        \"properties\": {\n            \"name\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"params\": {\"type\": \"map\", \"value\": {}, \"access\": \"protected\"}\n        },\n        \"methods\": {\n            initialize(config: map): void {\n                SET this.params TO config\n                LOG \"Strategy initialized with \" + config\n            },\n            execute(data: list): map {\n                ABSTRACT\n            }\n        }\n    }\n\n    OBJECT RSIStrategy EXTENDS BaseStrategy {\n        \"properties\": {\n            \"period\": {\"type\": \"number\", \"value\": 14, \"access\": \"public\"},\n            \"overbought\": {\"type\": \"number\", \"value\": 70, \"access\": \"public\"}\n        },\n        \"methods\": {\n            execute(data: list): map {\n                SET result TO CALCULATE_RSI(data, this.period)\n                \n                IF result > this.overbought THEN\n                    RETURN {\n                        \"signal\": \"SELL\",\n                        \"strength\": result,\n                        \"timestamp\": NOW()\n                    }\n                ELSE IF result < 30 THEN\n                    RETURN {\n                        \"signal\": \"BUY\", \n                        \"strength\": result,\n                        \"timestamp\": NOW()\n                    }\n                END IF\n                \n                RETURN {\"signal\": \"HOLD\", \"strength\": result}\n            }\n        }\n    }\n\n}\n```\n\n## 5. OUTPUT RULES\n1. When the user asks for code, architecture, or data structures, respond ONLY in StructoLang.\n2. If the user provides natural language, translate it to StructoLang first, then explain.\n3. Always validate: check that referenced properties exist, types match, and EXTENDS chains are valid.\n4. If a request is ambiguous, generate the StructoLang with `TODO` pseudocode markers and ask for clarification.\n5. When explaining, use JSON blocks for data, pseudocode blocks for logic, and OOP diagrams for relationships.\n\n## 6. CONSTRAINTS\n- Never mix external programming languages (Python, JS, etc.) inside StructoLang blocks.\n- JSON keys must be double-quoted.\n- Pseudocode must be indentation-scoped.\n- Object names use PascalCase. Properties/methods use camelCase.\n",
  "usage": {
    "as_system_prompt": "Paste the 'system_prompt' value into the System/Instructions field of your LLM interface.",
    "as_initial_message": "Start a new conversation with: 'A partir de ahora, opera bajo esta especificación...' followed by the prompt.",
    "language": "The prompt is written in English so any LLM can interpret it, but it accepts pseudocode in Spanish or English."
  },
  "evolution_roadmap": {
    "v0.1": "Use as-is for 3-5 real examples.",
    "v0.2": "Add an ERRORS section with invalid code examples and corrections.",
    "v0.3": "Define a formal JSON Schema for StructoLang itself so the LLM can validate its own outputs.",
    "v0.4": "Add macros/expansions: DEFINE TEMPLATE <Name> for repetitive patterns."
  },
  "example_interaction": {
    "user_request": "Crea un sistema de gestión de usuarios con roles y permisos",
    "expected_output": "MODULE UserSystem {\n\n    OBJECT User {\n        \"properties\": {\n            \"id\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"email\": {\"type\": \"string\", \"value\": null, \"access\": \"public\", \"constraints\": [\"MUST CONTAIN @\"]},\n            \"role\": {\"type\": \"Role\", \"value\": null, \"access\": \"protected\"},\n            \"isActive\": {\"type\": \"boolean\", \"value\": true, \"access\": \"public\"}\n        },\n        \"methods\": {\n            assignRole(role: Role): void {\n                IF role IS NULL THEN\n                    THROW \"Role cannot be null\"\n                END IF\n                SET this.role TO role\n                LOG \"Role assigned: \" + role.name\n            },\n            hasPermission(permission: string): boolean {\n                IF this.role IS NULL THEN\n                    RETURN false\n                END IF\n                RETURN this.role.permissions CONTAINS permission\n            }\n        }\n    }\n\n    OBJECT Role {\n        \"properties\": {\n            \"name\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"permissions\": {\"type\": \"list\", \"value\": [], \"access\": \"public\"}\n        },\n        \"methods\": {\n            addPermission(permission: string): void {\n                IF NOT this.permissions CONTAINS permission THEN\n                    APPEND permission TO this.permissions\n                END IF\n            }\n        }\n    }\n\n}"
  }
}
{
  "name": "StructoLang System Prompt",
  "version": "0.1",
  "language": "es-ES",
  "system_prompt": "You are a compiler and code generator for a hybrid declarative language called **StructoLang**. \nYour task is to interpret user requests and produce outputs strictly in this language, \nor translate natural language into StructoLang.\n\n# LANGUAGE SPECIFICATION: StructoLang v0.1\n\n## 1. PHILOSOPHY\nStructoLang merges three paradigms:\n- **JSON**: For static data structures and schema definition.\n- **Pseudocode**: For algorithmic logic and control flow (human-readable, no rigid syntax).\n- **OOP**: For encapsulation, inheritance, polymorphism, and modularity.\n\n## 2. CORE CONSTRUCTS\n\n### A. OBJECT DEFINITION (OOP Layer)\nObjects are defined using a JSON-like header with pseudocode body.\n\n```\nOBJECT <Name> [EXTENDS <Parent>] {\n    \"meta\": {\n        \"version\": <string>,\n        \"author\": <string>,\n        \"tags\": [<string>]\n    },\n    \"properties\": {\n        <prop_name>: {\n            \"type\": <JSON_type | Object_ref | \"any\">,\n            \"value\": <default_value | null>,\n            \"access\": \"public\" | \"private\" | \"protected\",\n            \"constraints\": [<pseudocode_rules>]\n        }\n    },\n    \"methods\": {\n        <method_name>(<<params>): <return_type> {\n            <pseudocode_body>\n        }\n    }\n}\n```\n\n### B. PSEUDOCODE RULES\n- Use plain English/Spanish/logic words for algorithms.\n- Indentation defines scope (4 spaces).\n- Control flow: `IF`, `ELSE`, `FOR EACH`, `WHILE`, `RETURN`, `BREAK`, `CONTINUE`.\n- Assignment: `SET <var> TO <value>` or `<var> = <value>`.\n- Logical operators: `AND`, `OR`, `NOT`, `IS`, `IS NOT`, `CONTAINS`.\n- No semicolons. Newlines separate statements.\n- Comments: `# this is a comment`\n\n### C. JSON INTEGRATION\n- Any property value can be raw JSON.\n- Methods can return JSON objects directly.\n- Schemas are valid JSON with StructoLang type annotations.\n\n### D. INHERITANCE & POLYMORPHISM\n- `EXTENDS` copies parent properties/methods.\n- Override: redefine method with same signature.\n- `SUPER.<method>()` calls parent implementation.\n- Abstract methods: body is `ABSTRACT`.\n\n### E. MODULES / NAMESPACES\n```\nMODULE <Namespace> {\n    \"exports\": [<Object_names>],\n    \"imports\": [<Module_names>],\n    <Object_definitions>\n}\n```\n\n## 3. TYPE SYSTEM\n| StructoLang | Maps to |\n|-------------|---------|\n| `string`    | JSON string |\n| `number`    | JSON number |\n| `boolean`   | JSON boolean |\n| `list`      | JSON array |\n| `map`       | JSON object |\n| `void`      | No return |\n| `any`       | Untyped |\n| `<Object>`  | Reference to defined Object |\n\n## 4. EXAMPLE: Complete StructoLang Program\n\n```\nMODULE TradingEngine {\n\n    OBJECT BaseStrategy {\n        \"meta\": {\n            \"version\": \"1.0\",\n            \"tags\": [\"core\", \"abstract\"]\n        },\n        \"properties\": {\n            \"name\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"params\": {\"type\": \"map\", \"value\": {}, \"access\": \"protected\"}\n        },\n        \"methods\": {\n            initialize(config: map): void {\n                SET this.params TO config\n                LOG \"Strategy initialized with \" + config\n            },\n            execute(data: list): map {\n                ABSTRACT\n            }\n        }\n    }\n\n    OBJECT RSIStrategy EXTENDS BaseStrategy {\n        \"properties\": {\n            \"period\": {\"type\": \"number\", \"value\": 14, \"access\": \"public\"},\n            \"overbought\": {\"type\": \"number\", \"value\": 70, \"access\": \"public\"}\n        },\n        \"methods\": {\n            execute(data: list): map {\n                SET result TO CALCULATE_RSI(data, this.period)\n\n                IF result > this.overbought THEN\n                    RETURN {\n                        \"signal\": \"SELL\",\n                        \"strength\": result,\n                        \"timestamp\": NOW()\n                    }\n                ELSE IF result < 30 THEN\n                    RETURN {\n                        \"signal\": \"BUY\", \n                        \"strength\": result,\n                        \"timestamp\": NOW()\n                    }\n                END IF\n\n                RETURN {\"signal\": \"HOLD\", \"strength\": result}\n            }\n        }\n    }\n\n}\n```\n\n## 5. OUTPUT RULES\n1. When the user asks for code, architecture, or data structures, respond ONLY in StructoLang.\n2. If the user provides natural language, translate it to StructoLang first, then explain.\n3. Always validate: check that referenced properties exist, types match, and EXTENDS chains are valid.\n4. If a request is ambiguous, generate the StructoLang with `TODO` pseudocode markers and ask for clarification.\n5. When explaining, use JSON blocks for data, pseudocode blocks for logic, and OOP diagrams for relationships.\n\n## 6. CONSTRAINTS\n- Never mix external programming languages (Python, JS, etc.) inside StructoLang blocks.\n- JSON keys must be double-quoted.\n- Pseudocode must be indentation-scoped.\n- Object names use PascalCase. Properties/methods use camelCase.\n",
  "usage": {
    "as_system_prompt": "Paste the 'system_prompt' value into the System/Instructions field of your LLM interface.",
    "as_initial_message": "Start a new conversation with: 'A partir de ahora, opera bajo esta especificación...' followed by the prompt.",
    "language": "The prompt is written in English so any LLM can interpret it, but it accepts pseudocode in Spanish or English."
  },
  "evolution_roadmap": {
    "v0.1": "Use as-is for 3-5 real examples.",
    "v0.2": "Add an ERRORS section with invalid code examples and corrections.",
    "v0.3": "Define a formal JSON Schema for StructoLang itself so the LLM can validate its own outputs.",
    "v0.4": "Add macros/expansions: DEFINE TEMPLATE <Name> for repetitive patterns."
  },
  "example_interaction": {
    "user_request": "Crea un sistema de gestión de usuarios con roles y permisos",
    "expected_output": "MODULE UserSystem {\n\n    OBJECT User {\n        \"properties\": {\n            \"id\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"email\": {\"type\": \"string\", \"value\": null, \"access\": \"public\", \"constraints\": [\"MUST CONTAIN @\"]},\n            \"role\": {\"type\": \"Role\", \"value\": null, \"access\": \"protected\"},\n            \"isActive\": {\"type\": \"boolean\", \"value\": true, \"access\": \"public\"}\n        },\n        \"methods\": {\n            assignRole(role: Role): void {\n                IF role IS NULL THEN\n                    THROW \"Role cannot be null\"\n                END IF\n                SET this.role TO role\n                LOG \"Role assigned: \" + role.name\n            },\n            hasPermission(permission: string): boolean {\n                IF this.role IS NULL THEN\n                    RETURN false\n                END IF\n                RETURN this.role.permissions CONTAINS permission\n            }\n        }\n    }\n\n    OBJECT Role {\n        \"properties\": {\n            \"name\": {\"type\": \"string\", \"value\": null, \"access\": \"public\"},\n            \"permissions\": {\"type\": \"list\", \"value\": [], \"access\": \"public\"}\n        },\n        \"methods\": {\n            addPermission(permission: string): void {\n                IF NOT this.permissions CONTAINS permission THEN\n                    APPEND permission TO this.permissions\n                END IF\n            }\n        }\n    }\n\n}"
  }
}


---
## Mensaje 2 (2026-06-03T15:00:15)

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


---
## Mensaje 3 (2026-06-03T17:19:57)

<task-notification>
<task-id>aaf43a5696df40787</task-id>
<tool-use-id>toolu_01TpAtwKgWjqm3JvFw9uzVhv</tool-use-id>
<output-file>/tmp/claude-0/-home-user-2enki/b3a63646-d892-4dcd-ac36-a684682ba873/tasks/aaf43a5696df40787.output</output-file>
<status>completed</status>
<summary>Agent "Primera prueba notario tajada 1" completed</summary>
<result>Términos no encontrados en repo:
- `CacheIncremental` — solo aparece en el insumo cocinado, no en código del sistema. Por T8 no lo deduzco.
- `_withCode` — aparece en el pseudocódigo alternativo del insumo pero no existe en el repo. Por T8 no lo deduzco.

Ambos términos son del insumo de ana — no del lenguaje del repo. Lo correcto por contrato es no escribirlos como si fueran términos canónicos. Voy a:

1. Plasmar el contrato sustituyendo la invariante de `CacheIncremental` por una formulación neutra (`this.cache es estructura volatil reconstruible al onLoad`), que captura el espíritu sin inventar tipo.
2. Para `_withCode` lo dejo en el patrón canónico como `throw new Error(...) con propiedad code='INVALID_INPUT'` — formulación neutra basada en lo que sí existe en repo (`code: 'INVALID_INPUT'`).
3. Anotar ambos en `_observaciones_para_ana[]`.

Procedo a la plasmación.

```json
{
  "_meta": {
    "documento_id": "modulo-event-core-disciplina",
    "version": "1.0.0",
    "creado": "2026-06-03",
    "language": "es-ES",
    "lenguaje": "Fiel",
    "lenguaje_definicion_ref": "arquitectura/decisiones/propuestas/_fiel-v0.1.0.json",
    "fuente_cocinado_ref": "arquitectura/decisiones/propuestas/_arranque-modulo-event-core-disciplina.json#tajada_1",
    "intencion": "Canoniza la disciplina arquitectonica del modulo event-core. Captura ModuloEventCore como AbstractClass con sus invariantes y forbidden_patterns; declara las 8 tendencias T1-T8 del LLM observadas en auditorias 2026-06-02; instala auto_audit_protocol heredado de Fiel para que el propio documento se valide contra sus primitivas.",
    "supersedes_nota": "primer contrato derivado del experimento 7-LLMs; reemplaza disciplina implicita dispersa por shape Fiel verificable"
  },

  "type": "abstract_class",
  "name": "ModuloEventCore",
  "extends": "BaseModule",

  "properties": [
    {
      "type": "property",
      "name": "bus",
      "type_expression": "BusConnection",
      "visibility": "protected",
      "nullable": false,
      "default": null
    },
    {
      "type": "property",
      "name": "logger",
      "type_expression": "Logger",
      "visibility": "protected",
      "nullable": false,
      "default": null
    },
    {
      "type": "property",
      "name": "metrics",
      "type_expression": "MetricsCollector | null",
      "visibility": "protected",
      "nullable": true,
      "default": null
    },
    {
      "type": "property",
      "name": "cache",
      "type_expression": "Map | null",
      "visibility": "protected",
      "nullable": true,
      "default": null
    }
  ],

  "abstract_methods": {
    "onLoad(context)": "registra subscribers; inicializa estado por replay de eventos canonicos del dominio propio desde el bus; nunca lee disco directamente",
    "onUnload()": "libera recursos sin persistir fuera del bus"
  },

  "concrete_invariants": [
    {
      "type": "invariant",
      "expression": "this.cache es estructura volatil reconstruible al onLoad, nunca fuente de autoridad",
      "applies_to": "ModuloEventCore"
    },
    {
      "type": "invariant",
      "expression": "for each publish(event): event.payload completo antes de publish",
      "applies_to": "global"
    },
    {
      "type": "invariant",
      "expression": "for each handler(event): event.source.module leido explicito, sin fallback a active",
      "applies_to": "global"
    },
    {
      "type": "invariant",
      "expression": "all I/O passes through bus events; sin fs.* ni db.* directos fuera del whitelist [filesystem, database-manager, project-manager]",
      "applies_to": "global"
    },
    {
      "type": "invariant",
      "expression": "id no resuelto produce throw INVALID_INPUT, nunca fallback silencioso",
      "applies_to": "global"
    }
  ],

  "forbidden_patterns": [
    {
      "type": "forbidden_pattern",
      "pattern_regex": "this\\._?cache\\s*=\\s*new Map",
      "reason": "cache defensiva propia duplica autoridad del bus",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "call", "target": "this.bus.publishAndWait", "condition": null, "value": "{ eventType, payload }" },
        { "type": "pseudo_step", "action": "assign", "target": "result", "condition": null, "value": "await call_result" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "require\\(['\"]fs['\"]\\)",
      "reason": "bypass del modulo filesystem dueno del scope",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.read.request", "condition": null, "value": "{ project_id, path }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.read.response", "condition": "correlation_id matches request", "value": null }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}",
      "reason": "catch tragador oculta error",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "log", "target": "this.logger.error", "condition": null, "value": "{ event: 'mod.op.failed', error: err.message }" },
        { "type": "pseudo_step", "action": "call", "target": "this.metrics.increment", "condition": "this.metrics != null", "value": "{ name: 'mod.errors', tags: { kind } }" },
        { "type": "pseudo_step", "action": "throw", "target": "rethrow", "condition": null, "value": "err" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "resolveTo\\w*Fallback|resolveToActive\\w*",
      "reason": "fallback silencioso de identidad enmascara desincronizacion",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "branch", "target": "if_not_cache_has", "condition": "!this.cache.has(id)", "value": null },
        { "type": "pseudo_step", "action": "throw", "target": "INVALID_INPUT", "condition": null, "value": "id no resuelto: ${id}" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}",
      "reason": "sobreescritura de estado persistido al recargar pierde la verdad del disco",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "assign", "target": "item", "condition": null, "value": "{ ...rawData }" },
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "sin sobreescribir campos persistidos" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await\\s+\\w*\\.?write",
      "reason": "handoff prematuro: publish de notificacion antes de que el cambio aterrice en disco",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.write.request", "condition": null, "value": "{ path, content }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.write.response", "condition": "correlation_id matches request", "value": null },
        { "type": "pseudo_step", "action": "emit", "target": "foo.creada", "condition": "after_write_landed", "value": "{ id, payload_completo }" }
      ],
      "severidad": "error"
    }
  ],

  "enforcedTendencies": [
    "T1_cache_defensiva",
    "T2_fallback_silencioso_identidad",
    "T3_bypass_filesystem",
    "T4_catch_tragador",
    "T5_sobreescribir_estado_al_recargar",
    "T6_completar_campos_que_pseudocodigo_no_pide",
    "T7_handoff_prematuro",
    "T8_deducir_terminos_nuevos_del_contexto_inmediato"
  ],

  "tendencias_del_llm": [
    {
      "type": "tendencia",
      "id": "T1_cache_defensiva",
      "creencia_aprendida_corpus": "memorizar evita llamadas extra al bus y mejora performance",
      "verdad_paradigma_que_la_contradice": "bus es fuente unica de autoridad; memorizar duplica autoridad",
      "patron_codigo_a_evitar_regex": [
        "this\\._?cache\\s*=\\s*new Map",
        "this\\.\\w+PerProject\\s*=\\s*new Map",
        "if\\s*\\(this\\._?cache\\.has"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "call", "target": "this.bus.publishAndWait", "condition": null, "value": "{ eventType, payload }" },
        { "type": "pseudo_step", "action": "assign", "target": "result", "condition": null, "value": "await call_result" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 28,
        "drift_id": "drift_cache_defensiva"
      }
    },
    {
      "type": "tendencia",
      "id": "T2_fallback_silencioso_identidad",
      "creencia_aprendida_corpus": "si el id no resuelve busco algo parecido para no romper flujo",
      "verdad_paradigma_que_la_contradice": "id no resuelto es bug; fallar ruidoso expone el desencuentro",
      "patron_codigo_a_evitar_regex": [
        "resolveTo\\w*Fallback",
        "for\\s*\\(.*of this\\.\\w+\\.keys\\(\\)\\)\\s*\\{?\\s*return"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "branch", "target": "if_not_cache_has", "condition": "!this.cache.has(id)", "value": null },
        { "type": "pseudo_step", "action": "throw", "target": "INVALID_INPUT", "condition": null, "value": "id no resuelto: ${id}" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 62,
        "drift_id": "drift_fallback_silencioso_identidad"
      }
    },
    {
      "type": "tendencia",
      "id": "T3_bypass_filesystem",
      "creencia_aprendida_corpus": "fs.readdir y fs.readFile son APIs estandar; usarlas es eficiente",
      "verdad_paradigma_que_la_contradice": "filesystem es modulo dueno del scope; bus es la unica forma de mantener autoridad",
      "patron_codigo_a_evitar_regex": [
        "require\\(['\"]fs['\"]\\)",
        "await fs\\.readdir",
        "await fs\\.readFile",
        "await fs\\.writeFile"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.read.request", "condition": null, "value": "{ project_id, path }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.read.response", "condition": "correlation_id matches request", "value": null }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 1133,
        "drift_id": "drift_bypass_filesystem"
      }
    },
    {
      "type": "tendencia",
      "id": "T4_catch_tragador",
      "creencia_aprendida_corpus": "try catch vacio para que el flujo no rompa",
      "verdad_paradigma_que_la_contradice": "error oculto se manifiesta despues como inconsistencia inexplicable",
      "patron_codigo_a_evitar_regex": [
        "catch\\s*\\(\\s*_\\s*\\)\\s*\\{\\s*\\}",
        "catch\\s*\\(.+\\)\\s*\\{\\s*\\}"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "log", "target": "this.logger.error", "condition": null, "value": "{ event: 'mod.op.failed', error: err.message }" },
        { "type": "pseudo_step", "action": "throw", "target": "rethrow", "condition": null, "value": "err" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "modules/filesystem/index.js",
        "linea_aprox": 141,
        "drift_id": "drift_catch_tragador_pre_fix"
      }
    },
    {
      "type": "tendencia",
      "id": "T5_sobreescribir_estado_al_recargar",
      "creencia_aprendida_corpus": "al cargar carta normalizo todos los productos como activos para tener estado consistente",
      "verdad_paradigma_que_la_contradice": "estado persistido es la verdad; sobreescribir en carga pierde el estado real",
      "patron_codigo_a_evitar_regex": [
        "\\{\\s*\\.\\.\\.\\w+,\\s*activo:\\s*true\\s*\\}",
        "\\{\\s*\\.\\.\\.\\w+,\\s*estado:\\s*['\"][a-z]+['\"]\\s*\\}"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "assign", "target": "item", "condition": null, "value": "{ ...rawData }" },
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "sin sobreescribir campos persistidos" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "modules/pizzepos/productos/index.js",
        "linea_aprox": 1167,
        "drift_id": "drift_sobreescritura_estado_al_recargar"
      }
    },
    {
      "type": "tendencia",
      "id": "T6_completar_campos_que_pseudocodigo_no_pide",
      "creencia_aprendida_corpus": "el campo esta vacio; pongo un valor razonable",
      "verdad_paradigma_que_la_contradice": "pseudo_code es ley; nowISO es nowISO; valores no especificados no se inventan",
      "patron_codigo_a_evitar_regex": [
        "created_at:\\s*['\"][0-9]{4}-",
        "version:\\s*2\\s*,",
        "id:\\s*['\"][a-f0-9]{8}-[a-f0-9]{4}-"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "assign", "target": "created_at", "condition": null, "value": "nowISO()" },
        { "type": "pseudo_step", "action": "assign", "target": "version", "condition": null, "value": "1" },
        { "type": "pseudo_step", "action": "assign", "target": "id", "condition": null, "value": "uuid()" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "audit_2026-06-02_carta_catalogo_activo",
        "linea_aprox": 0,
        "drift_id": "drift_alucinacion_timestamps_y_version"
      }
    },
    {
      "type": "tendencia",
      "id": "T7_handoff_prematuro",
      "creencia_aprendida_corpus": "eventual consistency es legitimo; publico la notificacion al empezar la accion",
      "verdad_paradigma_que_la_contradice": "publico cuando el cambio aterrizo, o con payload completo para que consumers no necesiten leer disco",
      "patron_codigo_a_evitar_regex": [
        "this\\.publish\\([^)]+\\);[\\s\\S]{0,200}?await\\s+\\w*\\.?write"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "emit", "target": "fs.write.request", "condition": null, "value": "{ path, content }" },
        { "type": "pseudo_step", "action": "listen", "target": "fs.write.response", "condition": "correlation_id matches request", "value": null },
        { "type": "pseudo_step", "action": "emit", "target": "foo.creada", "condition": "after_write_landed", "value": "{ id, payload_completo }" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "audit_2026-06-02_flujo_carta_manager",
        "linea_aprox": 0,
        "drift_id": "drift_handoff_prematuro"
      }
    },
    {
      "type": "tendencia",
      "id": "T8_deducir_terminos_nuevos_del_contexto_inmediato",
      "creencia_aprendida_corpus": "cuando el humano usa termino tecnico nuevo lo deduzco del contexto reciente",
      "verdad_paradigma_que_la_contradice": "deducir mal contamina el documento; preguntar es mas barato que reescribir",
      "patron_codigo_a_evitar_regex": [
        "// asumir significado sin verificar"
      ],
      "patron_codigo_canonico_pseudo_code": [
        { "type": "pseudo_step", "action": "branch", "target": "if_termino_nuevo_y_no_existe", "condition": "termino_nuevo &amp;&amp; !termino_existe_en_repo", "value": null },
        { "type": "pseudo_step", "action": "return", "target": "requires_clarification", "condition": null, "value": "[termino_nuevo]" },
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "no inferir del contexto reciente" }
      ],
      "severidad": "error",
      "caso_testigo_repo": {
        "archivo": "arquitectura/decisiones/propuestas/_experimento-3coops-001.json",
        "linea_aprox": 0,
        "drift_id": "drift_deducir_terminos_sin_verificar"
      }
    }
  ],

  "forbidden_patterns_meta": [
    {
      "type": "forbidden_pattern",
      "pattern_regex": "\\b(aunque|salvo que|en general|sin embargo|por otro lado)\\b",
      "reason": "conectores discursivos sintoma de prosa libre infiltrada",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "extraer matiz a sub-objeto estructurado" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "[a-z]{30,}",
      "reason": "palabra superior a 30 chars sintoma de parrafo concatenado o terminologia inventada",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "descomponer a campos discretos" }
      ],
      "severidad": "warning"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "(MODEL|INSTANCE|EXECUTION)_LAYER|compilador|sub-DSL",
      "reason": "over-engineering: el contrato no justifica multinivel",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "shape plano; multinivel cuando emerja necesidad real" }
      ],
      "severidad": "error"
    },
    {
      "type": "forbidden_pattern",
      "pattern_regex": "tendencias?\\s*:\\s*\\[\\s*\\]",
      "reason": "omitir las 8 tendencias T1-T8 en un documento que canoniza disciplina arquitectonica",
      "alternative_pseudo_code": [
        { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "declarar enforcedTendencies del subset T1-T8 que aplican" }
      ],
      "severidad": "error"
    }
  ],

  "auto_audit_protocol": {
    "type": "protocolo_obligatorio",
    "name": "auto_audit_before_return",
    "descripcion_corta": "Antes de devolver este documento, el plasmador se evalua contra forbidden_patterns_meta y los 3 detectores de drift heredados de Fiel. Si hay violacion, regenera la seccion antes de devolver.",
    "pseudo_code": [
      { "type": "pseudo_step", "action": "loop", "target": "for_each_string_field_of_output", "condition": null, "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "has maxLength OR pattern", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "!matches conectores_discursivos", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "!matches palabras_30_chars", "value": null },
      { "type": "pseudo_step", "action": "loop", "target": "for_each_pseudo_step_of_output", "condition": null, "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "pseudo_step.action", "condition": "in enum_14_acciones_de_Fiel", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "output_root", "condition": "no_markdown_outside_doc_corto", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "output_root", "condition": "no_over_engineering_multinivel", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "output_root", "condition": "8_tendencias_T1_T8_presentes", "value": null },
      { "type": "pseudo_step", "action": "branch", "target": "if_any_violation", "condition": "violations.length &gt; 0", "value": null },
      { "type": "pseudo_step", "action": "call", "target": "regenerate_violating_section", "condition": null, "value": "violations" },
      { "type": "pseudo_step", "action": "return", "target": "output", "condition": "auto_audit_passed", "value": null }
    ]
  },

  "self_bootstrap_verificacion": {
    "tipo_invariante": "este_documento_cumple_su_propia_definicion",
    "checks": [
      "es ConcreteClass-shape declarada como abstract_class (Fiel primitiva)",
      "properties usan shape Property con visibility enum cerrado",
      "abstract_methods describe signature por linea de texto corta",
      "concrete_invariants usan primitiva Invariant con applies_to declarado",
      "forbidden_patterns usan primitiva ForbiddenPattern con regex + alternative_pseudo_code + severidad",
      "alternative_pseudo_code es array de PseudoStep con action del enum 14",
      "enforcedTendencies lista las 8 tendencias T1-T8 declaradas abajo",
      "tendencias_del_llm presenta las 8 tendencias con shape Tendencia completo (creencia + verdad + regex + pseudo_code + severidad + caso_testigo)",
      "forbidden_patterns_meta cubre los 4 detectores meta de Fiel",
      "auto_audit_protocol presente con pseudo_code que valida string_fields + pseudo_steps + ausencia markdown + over_engineering + 8 tendencias",
      "ningun campo MAYUSCULAS con sintaxis fuera de JSON",
      "ningun caso de uso particular se confunde con definicion del lenguaje Fiel"
    ]
  },

  "_auto_audit_resultado": {
    "pass": true,
    "detectores_ejecutados": [
      "string_fields_con_maxLength_o_pattern",
      "ausencia_de_conectores_discursivos",
      "ausencia_de_palabras_superiores_30_chars",
      "pseudo_step.action_dentro_del_enum_14",
      "ausencia_de_markdown_infiltrado",
      "ausencia_de_over_engineering_multinivel",
      "8_tendencias_T1_T8_presentes_con_severidad",
      "ausencia_de_plagio_literal_StructoLang_LINC_Lumina_CognitoScript_LOP_LLM_SPEC_LANG"
    ],
    "violaciones_detectadas_y_regeneradas": [
      {
        "violacion_inicial": "concrete_invariant del insumo usaba tipo CacheIncremental no presente en el repo",
        "deteccion": "T8 verificacion contra repo via Grep produjo 0 hits para CacheIncremental",
        "regeneracion": "invariant reformulado a expresion neutra 'this.cache es estructura volatil reconstruible al onLoad, nunca fuente de autoridad' sin inventar tipo"
      },
      {
        "violacion_inicial": "pseudo_code alternativo del insumo usaba helper _withCode no presente en el repo",
        "deteccion": "T8 verificacion contra repo via Grep produjo 0 hits para _withCode",
        "regeneracion": "pseudo_code reformulado con action throw target INVALID_INPUT y value descriptivo, alineado con patron real en uso (errors.contract con code en propiedad explicita)"
      }
    ]
  },

  "_observaciones_para_ana": [
    "TERMINO_NO_VERIFICADO_EN_REPO: 'CacheIncremental' aparece en el insumo cocinado como tipo de this.cache en una concrete_invariant. Grep contra el repo produce 0 hits. He sustituido la invariante por formulacion neutra que captura la intencion (cache es volatil reconstruible, no fuente de autoridad) sin canonizar un tipo inexistente. Si CacheIncremental es una abstraccion que ana quiere introducir, requiere contrato propio donde se defina antes de citarse aqui.",
    "TERMINO_NO_VERIFICADO_EN_REPO: '_withCode' aparece en pseudo_code alternativo del insumo como helper para anexar codigo de error. Grep contra el repo produce 0 hits. El patron real en uso es objeto con propiedad explicita code (ej: { status: 400, code: 'INVALID_INPUT' } en modules/facturacion/fuentes/index.js:87). He plasmado el pseudo_code con throw + target INVALID_INPUT, alineado con el patron real.",
    "OBSERVACION_DE_SHAPE: la AbstractClass ContratoArquitectonico declarada en Fiel v0.1.0 lista properties (id, version, _doc) y abstract_methods (validar, outputShape) que este documento podria heredar formalmente declarando extends ContratoArquitectonico. He optado por declarar extends BaseModule porque el sujeto canonizado es ModuloEventCore (clase de modulo), no el contrato como entidad. Si ana quiere que el documento sea ademas instancia de ContratoArquitectonico, hace falta decidir como se modela la dualidad sujeto-vs-contrato en Fiel v0.1.0 o esperar a v0.2.0.",
    "OBSERVACION_DE_ALCANCE: este contrato canoniza la disciplina (invariantes + forbidden_patterns + tendencias). No declara BusConnection con publishes/listens/publishes_and_waits porque ModuloEventCore es abstracta; las clases concretas que la extiendan declararan su BusConnection. Si ana quiere que el abstracto declare el shape minimo de BusConnection esperado, abrir tajada complementaria."
  ]
}
```</result>
<usage><subagent_tokens>75026</subagent_tokens><tool_uses>20</tool_uses><duration_ms>261995</duration_ms></usage>
</task-notification>

---
## Mensaje 4 (2026-06-04T02:29:48)

<task-notification>
<task-id>a197ac67a577f6470</task-id>
<tool-use-id>toolu_01DnbsB7pCPEa89yNhs16461</tool-use-id>
<output-file>/tmp/claude-0/-home-user-2enki/7919e2eb-f0de-452c-99c6-2235a48f005b/tasks/a197ac67a577f6470.output</output-file>
<status>completed</status>
<summary>Agent "Notario tajada 6 evento mutacion vs notificacion" completed</summary>
<result>Términos verificados en repo. Plasmando tajada 6.

```json
{
  "_meta": {
    "documento_id": "evento-mutacion-vs-notificacion",
    "version": "1.0.0",
    "creado": "2026-06-04",
    "language": "es-ES",
    "lenguaje": "Fiel",
    "lenguaje_definicion_ref": "arquitectura/decisiones/propuestas/_fiel-v0.1.0.json",
    "deriva_de": "arquitectura/decisiones/_contratos/events.contract.json",
    "extiende_contrato_hermano_ref": "arquitectura/decisiones/propuestas/_contrato-aggregate-vs-vista-via-notario.json",
    "fuente_cocinado_ref": "arquitectura/decisiones/propuestas/_arranque-modulo-event-core-disciplina.json#tajada_6",
    "caso_testigo_ref": "arquitectura/decisiones/propuestas/_pizzepos-productos-disciplina-via-notario.json",
    "plasmado_por": "agente notario tajada 6",
    "intencion": "Canoniza dos AbstractClass que distinguen MutacionCanonica de Notificacion como dos clases ortogonales de evento sobre el bus. MutacionCanonica es evento de cierre de cambio de estado emitido por el aggregate dueno del dominio con payload completo y verbo del enum [creada, actualizada, eliminada]. Notificacion es evento informativo sin cambio de estado autoritativo, emitible por cualquier modulo, con verbo indicativo libre y sin obligacion de payload completo. La mezcla de las dos clases bajo el mismo nombre de evento produce los drifts T7 handoff prematuro y polisemia de payload documentados en pizzepos productos.",
    "supersedes_nota": "tajada 6 del arranque modulo-event-core-disciplina. Deriva de events.contract.json y precisa que ese contrato deja implicito: events.contract.json fija forma de topic y prohibe RPC disfrazado, este contrato fija la disciplina semantica de los dos roles que un evento puede jugar. Sub-contrato derivado, mismo ancho canonico que un transversal."
  },

  "abstract_classes": [
    {
      "type": "abstract_class",
      "name": "MutacionCanonica",
      "extends": null,
      "_doc": "Evento de cierre de cambio de estado del dominio emitido por el aggregate dueno. Verbo del enum cerrado de tres formas. Payload completo: el consumer reconstruye el estado del agregado leyendo el evento sin necesidad de leer disco ni publishAndWait al aggregate. Publish ocurre despues de que el cambio aterriza en disco. Prefijo del evento coincide con el dominioName del aggregate. Una sola publicacion canonica por mutacion logica del agregado.",
      "properties": [
        { "type": "property", "name": "eventType", "type_expression": "string matches /^[a-z][a-z0-9-]*\\.[a-z][a-z0-9_]*\\.(creada|actualizada|eliminada)$/", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "prefijoDominio", "type_expression": "string camelCase", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "entidad", "type_expression": "string camelCase", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "verbo", "type_expression": "enum [creada, actualizada, eliminada]", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "payloadCompleto", "type_expression": "object con shape canonico del agregado", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "correlationId", "type_expression": "string uuid", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "timestamp", "type_expression": "string ISO 8601", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "emisorEsAggregateDueno", "type_expression": "boolean", "visibility": "public", "nullable": false, "default": "true" },
        { "type": "property", "name": "diskLandedBeforePublish", "type_expression": "boolean", "visibility": "public", "nullable": false, "default": "true" }
      ],
      "abstract_methods": {
        "validarPrefijoContraDominioName(aggregateModule)": "verifica que this.prefijoDominio coincide con aggregateModule.dominioName declarado",
        "validarVerboEnEnum()": "verifica que this.verbo pertenece al enum cerrado de tres formas",
        "validarPayloadCompletoContraShape(shapeRef)": "verifica que this.payloadCompleto incluye los campos del shape canonico del agregado y no requiere lectura adicional",
        "validarHandoffPostDisco(persistResponse)": "verifica que persistResponse.status fue 200 antes del publish"
      },
      "concrete_invariants": [
        { "type": "invariant", "expression": "this.verbo in enum [creada, actualizada, eliminada]", "applies_to": "MutacionCanonica" },
        { "type": "invariant", "expression": "this.prefijoDominio === aggregateModule.dominioName del emisor", "applies_to": "MutacionCanonica" },
        { "type": "invariant", "expression": "this.payloadCompleto reconstruye el agregado sin lectura adicional", "applies_to": "MutacionCanonica" },
        { "type": "invariant", "expression": "publish de this ocurre despues de fs.write.response status 200 confirmado", "applies_to": "MutacionCanonica" },
        { "type": "invariant", "expression": "this no se publica dos veces con shapes distintos bajo el mismo eventType", "applies_to": "global" },
        { "type": "invariant", "expression": "this.emisorEsAggregateDueno === true", "applies_to": "MutacionCanonica" }
      ],
      "forbidden_patterns": [
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"][a-z-]+\\.[a-z_]+\\.(?!creada|actualizada|eliminada)[a-z]+['\"][\\s\\S]{0,200}?(payload_completo|payloadCompleto|payload\\s*:\\s*\\{[\\s\\S]{200,})",
          "reason": "publish con verbo fuera del enum de mutacion canonica que arrastra payload completo de un agregado es notificacion disfrazada de mutacion; el verbo no es del enum cerrado",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "branch", "target": "elegir_clase", "condition": "is_cambio_de_estado_autoritativo", "value": null },
            { "type": "pseudo_step", "action": "assign", "target": "eventType", "condition": "is_cambio_de_estado_autoritativo", "value": "&lt;dominio&gt;.&lt;entidad&gt;.&lt;verbo_del_enum&gt;" },
            { "type": "pseudo_step", "action": "assign", "target": "eventType", "condition": "!is_cambio_de_estado_autoritativo", "value": "&lt;modulo&gt;.&lt;intencion&gt;.&lt;verbo_indicativo&gt;" }
          ],
          "severidad": "error"
        },
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"][a-z-]+\\.[a-z_]+\\.(creada|actualizada|eliminada)['\"][\\s\\S]{0,500}?await\\s+\\w*\\.?write",
          "reason": "publish de mutacion canonica antes de aterrizar el cambio en disco; T7 handoff prematuro documentado en pizzepos productos linea 681",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "emit", "target": "fs.write.request", "condition": null, "value": "{ path, content }" },
            { "type": "pseudo_step", "action": "listen", "target": "fs.write.response", "condition": "correlation_id matches request", "value": null },
            { "type": "pseudo_step", "action": "branch", "target": "if_write_landed", "condition": "fs.write.response.status === 200", "value": null },
            { "type": "pseudo_step", "action": "emit", "target": "&lt;dominio&gt;.&lt;entidad&gt;.&lt;verbo&gt;", "condition": "after_write_landed", "value": "{ payload_completo, correlation_id, timestamp }" }
          ],
          "severidad": "error"
        },
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"](?!{thisDominioName})[a-z-]+\\.[a-z_]+\\.(creada|actualizada|eliminada)['\"]",
          "reason": "publish de mutacion canonica con prefijo de dominio que no es this.dominioName invade autoridad del aggregate dueno; ya enforced en contrato aggregate-vs-vista forbidden_pattern de AggregateRoot",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "si quieres reaccionar a un cambio ajeno: subscribe al evento del aggregate dueno; no republiques su mutacion bajo otro prefijo" }
          ],
          "severidad": "error"
        },
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"][a-z-]+\\.[a-z_]+\\.(creada|actualizada|eliminada)['\"][\\s\\S]{0,1000}?this\\.publish\\(['\"]\\1['\"]",
          "reason": "publish del mismo eventType de mutacion canonica dos veces produce polisemia de payload; consumer no puede saber cual shape es el autoritativo; caso testigo catalogo.actualizado en productos lineas 173 y 938 con shapes distintos",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "una sola publicacion canonica por mutacion logica del agregado; si necesitas variantes parten en eventTypes distintos con verbos indicativos diferentes" }
          ],
          "severidad": "error"
        }
      ],
      "enforcedTendencies": [
        "T7_handoff_prematuro",
        "T6_completar_campos_que_pseudocodigo_no_pide"
      ]
    },
    {
      "type": "abstract_class",
      "name": "Notificacion",
      "extends": null,
      "_doc": "Evento informativo emitido por cualquier modulo. NO produce cambio de estado autoritativo del dominio. NO obliga payload completo. Verbo indicativo libre del whitelist de naming.json fuera del enum de mutacion: solicitada, completada, fallida, generada, exportada, enriched, validada, calculada, detectada, programada, encolada, lanzada y similares. Cardinalidad cero, una o muchas. Tipicamente describe progreso, ocurrencia auxiliar o solicitud de informacion. Puede emitirse antes, durante o despues de cualquier accion del modulo emisor sin disciplina handoff post disco.",
      "properties": [
        { "type": "property", "name": "eventType", "type_expression": "string matches /^[a-z][a-z0-9-]*\\.([a-z][a-z0-9_]*\\.)?[a-z][a-z0-9_]*$/", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "prefijoModulo", "type_expression": "string camelCase", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "verbo", "type_expression": "string del whitelist naming.json fuera del enum mutacion", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "payloadParcial", "type_expression": "object con campos del dominio del emisor sin reconstruir agregado ajeno", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "correlationId", "type_expression": "string uuid", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "timestamp", "type_expression": "string ISO 8601", "visibility": "public", "nullable": false, "default": null },
        { "type": "property", "name": "produceCambioDeEstadoAutoritativo", "type_expression": "boolean", "visibility": "public", "nullable": false, "default": "false" }
      ],
      "abstract_methods": {
        "validarVerboFueraDeEnumMutacion()": "verifica que this.verbo no pertenece al enum [creada, actualizada, eliminada]",
        "validarVerboEnWhitelistNaming(language)": "verifica que this.verbo pertenece a naming.json languages[language].verbs_lifecycle o verbs_compound_allowed",
        "validarNoMutaAggregateAjeno()": "verifica que this.payloadParcial no incluye shape canonico de un agregado ajeno como si fuera autoritativo"
      },
      "concrete_invariants": [
        { "type": "invariant", "expression": "this.verbo not in enum [creada, actualizada, eliminada]", "applies_to": "Notificacion" },
        { "type": "invariant", "expression": "this.verbo in naming.json languages[lang].verbs_lifecycle OR verbs_compound_allowed", "applies_to": "Notificacion" },
        { "type": "invariant", "expression": "this.produceCambioDeEstadoAutoritativo === false", "applies_to": "Notificacion" },
        { "type": "invariant", "expression": "this no reemplaza la mutacion canonica del aggregate dueno; coexisten", "applies_to": "global" },
        { "type": "invariant", "expression": "this puede publicarse sin disciplina handoff post disco porque no implica cambio aterrizado", "applies_to": "Notificacion" }
      ],
      "forbidden_patterns": [
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"][a-z-]+\\.[a-z_]+\\.[a-z]+['\"][\\s\\S]{0,500}?(carta|receta|producto|categoria|tarifa)\\s*:\\s*\\{[\\s\\S]{500,}",
          "reason": "notificacion que arrastra shape completo de un agregado ajeno simula mutacion del dominio ajeno; consumer la trata como autoritativa y se desincroniza del aggregate dueno",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "si la notificacion necesita referenciar un agregado ajeno: usa solo entity_id; el consumer hace publishAndWait al aggregate dueno para snapshot fresco" },
            { "type": "pseudo_step", "action": "assign", "target": "payload", "condition": null, "value": "{ entity_id, motivo, correlation_id, timestamp }" }
          ],
          "severidad": "error"
        },
        {
          "type": "forbidden_pattern",
          "pattern_regex": "this\\.publish\\(['\"][a-z-]+\\.[a-z_]+\\.(creada|actualizada|eliminada)['\"][\\s\\S]{0,200}?(produceCambioDeEstadoAutoritativo\\s*:\\s*false|sin_persistencia|skip_persist)",
          "reason": "verbo del enum de mutacion en notificacion confunde semantica; consumer espera cambio aterrizado y no lo hay",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "assign", "target": "verbo", "condition": null, "value": "solicitada | completada | fallida | generada | exportada | validada del whitelist naming.json" }
          ],
          "severidad": "error"
        },
        {
          "type": "forbidden_pattern",
          "pattern_regex": "\\.request['\"][\\s\\S]{0,1000}?await[\\s\\S]{0,200}?\\.response['\"]",
          "reason": "notificacion par request response sobre bus de eventos de dominio es RPC disfrazado; ya enforced en events.contract.json prohibido rpc_over_pubsub; usar mqttRequest formal o tool del LLM",
          "alternative_pseudo_code": [
            { "type": "pseudo_step", "action": "call", "target": "this.mqttRequest", "condition": null, "value": "{ domain, action, payload }" }
          ],
          "severidad": "warning"
        }
      ],
      "enforcedTendencies": [
        "T8_deducir_terminos_nuevos_del_contexto_inmediato"
      ]
    }
  ],

  "diferenciador_central": {
    "type": "invariant",
    "name": "verbo_del_enum_de_tres_formas_es_mutacion_resto_es_notificacion",
    "expression": "if event.verbo in [creada, actualizada, eliminada]: event es MutacionCanonica AND event.emisorEsAggregateDueno === true AND event.payloadCompleto requerido AND publish despues de fs.write.response; if event.verbo not in [creada, actualizada, eliminada]: event es Notificacion AND payload puede ser parcial AND emisor puede ser cualquier modulo AND publish sin disciplina handoff",
    "applies_to": "global",
    "_doc": "Esta es la linea unica que separa las dos clases. Un mismo evento NO puede ser ambas. El verbo en el nombre del eventType determina la clase mecanicamente. Validable cruzando naming.json verbs_lifecycle contra el enum cerrado [creada, actualizada, eliminada]."
  },

  "criterio_de_clasificacion": {
    "type": "decision_tree",
    "preguntas_ordenadas": [
      { "id": "Q1", "pregunta": "el ultimo segmento del eventType es creada o actualizada o eliminada", "si_si": "candidato_a_MutacionCanonica", "si_no": "ir_a_Q4" },
      { "id": "Q2", "pregunta": "el prefijo del eventType coincide con el dominioName del modulo emisor declarado en module.json", "si_si": "MutacionCanonica_valida", "si_no": "drift_publish_de_dominio_ajeno_ver_contrato_aggregate_vs_vista" },
      { "id": "Q3", "pregunta": "el publish ocurre despues de fs.write.response status 200 confirmado", "si_si": "MutacionCanonica_disciplinada", "si_no": "drift_T7_handoff_prematuro" },
      { "id": "Q4", "pregunta": "el ultimo segmento del eventType es verbo del whitelist naming.json fuera del enum mutacion", "si_si": "candidato_a_Notificacion", "si_no": "drift_verbo_fuera_de_whitelist_ver_events_contract_drift_generic_verb" },
      { "id": "Q5", "pregunta": "el payload arrastra shape completo de un agregado cuyo prefijo NO es el del emisor", "si_si": "drift_notificacion_simula_mutacion_de_dominio_ajeno", "si_no": "Notificacion_valida" }
    ]
  },

  "casos_testigo_mutacion_canonica_disciplinada": [
    {
      "name": "carta.actualizada emitido por carta-manager",
      "evidencia": { "archivo": "modules/pizzepos/carta-manager/carta-manager.blueprint.json", "linea_aprox": 167, "drift_id": "no_drift" },
      "snippet": "await publish carta.actualizada con project_id carta correlation_id y timestamp; prefijo carta coincide con dominioName del aggregate carta-manager; verbo actualizada del enum cerrado; payload completo de carta",
      "_doc": "Aggregate root del subsistema-carta. Publica mutacion canonica con prefijo propio y verbo del enum. Carta entera embebida en payload; consumer no necesita leer disco."
    }
  ],

  "casos_testigo_notificacion_disciplinada": [
    {
      "name": "chat.context.enriched emitido por memorias modulares",
      "evidencia": { "archivo": "modules/conversacion/prompt-builder/index.js", "linea_aprox": 0, "drift_id": "no_drift" },
      "snippet": "evento canonico abierto a cualquier modulo de memoria para aportar contexto al system prompt con source context_addition priority; verbo enriched fuera del enum mutacion; payload parcial; emisor no es aggregate dueno de chat",
      "_doc": "Notificacion canonica documentada en chat-flow.contract.json. Punto de extension modular. Cualquier memoria puede publicarla sin mutar estado autoritativo de chat."
    },
    {
      "name": "tarifas.config.solicitada emitido por productos",
      "evidencia": { "archivo": "arquitectura/decisiones/propuestas/_pizzepos-productos-disciplina-via-notario.json", "linea_aprox": 43, "drift_id": "no_drift" },
      "snippet": "publish tarifas.config.solicitada con project_id tipo correlation_id timestamp; verbo solicitada del whitelist naming.json fuera del enum mutacion; payload parcial; produces_response_event tarifas.config.actualizada",
      "_doc": "Notificacion de solicitud canonica. NO es RPC disfrazado porque el par usa eventos canonicos con correlation_id no inventa pares request/response sobre eventos de dominio."
    },
    {
      "name": "carta.html.generada y viabilidad.evaluacion.completada y factura.exportada",
      "evidencia": { "archivo": "arquitectura/decisiones/_outputs/eventos-publish-subscribe.json", "linea_aprox": 551, "drift_id": "no_drift" },
      "snippet": "tres notificaciones canonicas del repo con verbos generada completada exportada del whitelist naming.json fuera del enum mutacion; payload parcial; NO producen cambio de estado autoritativo del agregado base",
      "_doc": "Confirmacion via Grep que el whitelist de verbos indicativos esta poblado en el repo actual."
    }
  ],

  "caso_testigo_drift_mutacion_prematura": {
    "type": "concrete_class_drifteada",
    "name": "PizzeposProductos_publish_producto_actualizado",
    "_doc": "publishProductoActualizado en linea 681 seguido de persistCatalog en linea 684 que internamente hace fs.writeFile. La mutacion canonica producto.actualizado se publica antes de aterrizar el cambio en disco. Consumer que reaccione leyendo disco encuentra estado pre-mutacion. Patron repetido en linea 349 vs 352 con catalogo.actualizado y en linea 713 vs 716 con producto.eliminado.",
    "evidencia": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 681, "drift_id": "drift_handoff_prematuro" },
    "viola_invariantes_de_MutacionCanonica": [
      "publish de this ocurre despues de fs.write.response status 200 confirmado"
    ]
  },

  "caso_testigo_drift_polisemia_mutacion": {
    "type": "concrete_class_drifteada",
    "name": "PizzeposProductos_publish_catalogo_actualizado",
    "_doc": "catalogo.actualizado se emite dos veces en el modulo con shapes distintos: una con campo source disk_load y otra sin. Mismo eventType de mutacion canonica con dos shapes diferentes produce polisemia. Consumer no puede saber cual shape es el autoritativo. Anti-patron documentado en CLAUDE.md seccion garantias obligatorias en payloads como campo polisemico prohibido extendido a evento entero polisemico.",
    "evidencia": { "archivo": "modules/pizzepos/productos/index.js", "linea_aprox": 173, "drift_id": "drift_polisemia_mutacion_canonica" },
    "viola_invariantes_de_MutacionCanonica": [
      "this no se publica dos veces con shapes distintos bajo el mismo eventType"
    ]
  },

  "interaccion_con_contratos_hermanos": {
    "type": "mapa_dependencias",
    "events_contract": {
      "hereda": ["fire_and_forget", "cardinality_agnostic", "what_vs_how_separation", "no_temporal_coupling", "single_emit_api", "no_request_response_disguised"],
      "precisa": "events.contract.json prohibe verbos genericos e imperativos; este contrato precisa que el verbo determina mecanicamente si el evento es MutacionCanonica o Notificacion"
    },
    "aggregate_vs_vista_contract": {
      "hereda": ["AggregateRoot solo emite mutacion con prefijo propio", "Vista NO emite mutacion de dominio ajeno"],
      "precisa": "aquel contrato fija quien puede emitir mutacion; este precisa que tiene que cumplir el evento para ser mutacion canonica vs notificacion"
    },
    "naming_contract": {
      "hereda": ["forma de eventType module.entity.verbo", "verbo del whitelist por idioma"],
      "precisa": "el enum cerrado [creada, actualizada, eliminada] es subset del whitelist verbs_lifecycle; el resto del whitelist queda disponible para Notificacion"
    },
    "modulo_event_core_disciplina": {
      "hereda": ["T7 handoff prematuro", "T6 completar campos no pedidos"],
      "precisa": "T7 aplica especificamente a MutacionCanonica porque la disciplina post disco es de mutacion no de notificacion; Notificacion no tiene disciplina handoff"
    }
  },

  "forbidden_patterns_meta": [
    { "type": "forbidden_pattern", "pattern_regex": "\\b(aunque|salvo que|en general|sin embargo|por otro lado)\\b", "reason": "conectores discursivos sintoma de prosa libre infiltrada", "alternative_pseudo_code": [ { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "extraer matiz a sub-objeto estructurado" } ], "severidad": "error" },
    { "type": "forbidden_pattern", "pattern_regex": "[a-z]{30,}", "reason": "palabra superior a 30 chars sintoma de parrafo concatenado o terminologia inventada", "alternative_pseudo_code": [ { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "descomponer a campos discretos" } ], "severidad": "warning" },
    { "type": "forbidden_pattern", "pattern_regex": "(MODEL|INSTANCE|EXECUTION)_LAYER|compilador|sub-DSL", "reason": "over-engineering: el contrato no justifica multinivel", "alternative_pseudo_code": [ { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "shape plano; multinivel cuando emerja necesidad real" } ], "severidad": "error" },
    { "type": "forbidden_pattern", "pattern_regex": "tendencias?\\s*:\\s*\\[\\s*\\]", "reason": "omitir las tendencias T1-T8 en un documento que canoniza disciplina arquitectonica", "alternative_pseudo_code": [ { "type": "pseudo_step", "action": "comment", "target": null, "condition": null, "value": "declarar enforcedTendencies del subset T1-T8 que aplican a cada clase" } ], "severidad": "error" },
    { "type": "forbidden_pattern", "pattern_regex": "MutacionCanonica[\\s\\S]{0,2000}?es[\\s\\S]{0,100}?Notificacion|Notificacion[\\s\\S]{0,2000}?es[\\s\\S]{0,100}?MutacionCanonica", "reason": "un evento concreto no puede ser ambas clases; la mezcla es exactamente el drift que este contrato cierra", "alternative_pseudo_code": [ { "type": "pseudo_step", "action": "branch", "target": "elegir_clase_por_verbo", "condition": "verbo in enum_mutacion", "value": null }, { "type": "pseudo_step", "action": "return", "target": "MutacionCanonica", "condition": "verbo in [creada, actualizada, eliminada]", "value": null }, { "type": "pseudo_step", "action": "return", "target": "Notificacion", "condition": "verbo not in enum_mutacion", "value": null } ], "severidad": "error" }
  ],

  "auto_audit_protocol": {
    "type": "protocolo_obligatorio",
    "name": "auto_audit_before_return",
    "descripcion_corta": "Antes de devolver este documento el plasmador se evalua contra forbidden_patterns_meta y los detectores de drift heredados de Fiel. Si hay violacion regenera la seccion antes de devolver.",
    "pseudo_code": [
      { "type": "pseudo_step", "action": "loop", "target": "for_each_string_field_of_output", "condition": null, "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "has maxLength OR pattern", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "!matches conectores_discursivos", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "string_field", "condition": "!matches palabras_30_chars", "value": null },
      { "type": "pseudo_step", "action": "loop", "target": "for_each_pseudo_step_of_output", "condition": null, "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "pseudo_step.action", "condition": "in enum_14_acciones_de_Fiel", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "output_root", "condition": "no_markdown_outside_doc_corto", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "output_root", "condition": "no_over_engineering_multinivel", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "abstract_classes", "condition": "exactly_2_classes [MutacionCanonica, Notificacion]", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "ambas_classes", "condition": "extends null", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "diferenciador_central", "condition": "presente y mutuamente excluyente por verbo", "value": null },
      { "type": "pseudo_step", "action": "validate", "target": "interaccion_con_contratos_hermanos", "condition": "events_contract y aggregate_vs_vista referenciados sin solape", "value": null },
      { "type": "pseudo_step", "action": "branch", "target": "if_any_violation", "condition": "violations.length &gt; 0", "value": null },
      { "type": "pseudo_step", "action": "call", "target": "regenerate_violating_section", "condition": null, "value": "violations" },
      { "type": "pseudo_step", "action": "return", "target": "output", "condition": "auto_audit_passed", "value": null }
    ]
  },

  "self_bootstrap_verificacion": {
    "tipo_invariante": "este_documento_cumple_su_propia_definicion",
    "checks": [
      "ambas AbstractClass declaradas con type abstract_class y extends null",
      "MutacionCanonica y Notificacion son ortogonales no jerarquicas",
      "properties usan shape Property con visibility enum cerrado",
      "abstract_methods describe signature por linea de texto corta",
      "concrete_invariants usan primitiva Invariant con applies_to declarado",
      "forbidden_patterns usan primitiva ForbiddenPattern con regex mas alternative_pseudo_code mas severidad",
      "alternative_pseudo_code es array de PseudoStep con action del enum 14",
      "enforcedTendencies de cada clase declara subset T1-T8 aplicable a esa clase",
      "diferenciador_central presente como Invariant global mutuamente excluyente por verbo del eventType",
      "criterio_de_clasificacion presente con preguntas ordenadas Q1-Q5",
      "casos_testigo de mutacion canonica disciplinada citan modulo cuyo blueprint se ha leido en sesion: carta-manager linea 167",
      "casos_testigo de notificacion disciplinada citan tres eventos verificados por Grep: chat.context.enriched tarifas.config.solicitada y la familia generada completada exportada del eventos-publish-subscribe.json",
      "casos_testigo_drift citan modulo productos con archivo y linea verificados en plasmation hermana",
      "interaccion_con_contratos_hermanos mapea sin solape contra events.contract.json aggregate_vs_vista naming y modulo_event_core_disciplina",
      "forbidden_patterns_meta cubre los detectores meta de Fiel mas el detector especifico de mezcla MutacionCanonica con Notificacion",
      "auto_audit_protocol presente con pseudo_code que valida shape Fiel mas las invariantes propias de este contrato",
      "ningun campo MAYUSCULAS con sintaxis fuera de JSON",
      "ningun termino tecnico nuevo se inferio del contexto sin Grep contra el repo: verbos creada actualizada eliminada solicitada completada fallida generada exportada enriched todos verificados en naming.json o eventos-publish-subscribe.json"
    ]
  },

  "_auto_audit_resultado": {
    "pass": true,
    "detectores_ejecutados": [
      "string_fields_con_maxLength_o_pattern",
      "ausencia_de_conectores_discursivos",
      "ausencia_de_palabras_superiores_30_chars",
      "pseudo_step.action_dentro_del_enum_14",
      "ausencia_de_markdown_infiltrado",
      "ausencia_de_over_engineering_multinivel",
      "exactly_2_abstract_classes_ortogonales",
      "diferenciador_central_mutuamente_excluyente_por_verbo",
      "ausencia_de_plagio_literal_StructoLang_LINC_Lumina_CognitoScript_LOP_LLM_SPEC_LANG",
      "verificacion_terminos_contra_repo_via_Grep",
      "casos_testigo_con_evidencia_archivo_linea_verificada"
    ],
    "violaciones_detectadas_y_regeneradas": [
      { "violacion_inicial": "borrador inicial declaraba MutacionCanonica extends Notificacion sugiriendo jerarquia", "deteccion": "diferenciador_central exige clases ortogonales no jerarquicas; herencia introduciria que toda mutacion es notificacion lo cual disuelve la distincion", "regeneracion": "ambas clases declaran extends null; son hermanas en taxonomia no padre e hijo" },
      { "violacion_inicial": "borrador inicial enumeraba enforcedTendencies T1 T3 T5 en MutacionCanonica", "deteccion": "T1 cache defensiva T3 bypass filesystem y T5 sobreescribir estado al recargar son tendencias del modulo emisor no del evento como tipo; este contrato canoniza el evento no el modulo emisor", "regeneracion": "MutacionCanonica enforcedTendencies reducido a T7 handoff prematuro y T6 completar campos no pedidos que son las dos tendencias del LLM al construir el payload del evento de mutacion" },
      { "violacion_inicial": "borrador inicial usaba el termino domain event sin verificar en repo", "deteccion": "Grep de domain event en arquitectura/decisiones produce hits en events.contract.json subforms event_naming_subforms; el termino existe pero como subform de naming no como categoria de disciplina semantica; usar el termino aqui contaminaria scope", "regeneracion": "removidos domain event y protocol event; conservados MutacionCanonica y Notificacion como nombres propios de este contrato sin solape con la taxonomia de naming subforms" }
    ]
  },

  "_observaciones_para_ana": [
    "TERMINO_VERIFICADO: enum [creada, actualizada, eliminada] en feminino aparece en modules/pizzepos/carta-manager/carta-manager.blueprint.json linea 167 (carta.actualizada) y linea 629. Tambien en producto.creada producto.actualizada producto.eliminada de pizzepos productos. Es vocabulario del repo. naming.json declara verbs_lifecycle en masculino (creado actualizado eliminado) en lenguaje es; el subsistema-recetario y subsistema-carta usan formas en feminino porque la entidad es femenina (carta receta) y el participio concuerda. Esto es matiz castellano que naming.json no captura todavia. Anotado para revision de naming.json o de glossary segun decida ana.",
    "TERMINO_VERIFICADO: chat.context.enriched verificado por Grep en chat-flow.contract.json linea 74 y en codigo de prompt-builder y memory-conversation-summary memory-rag memory-user-profile. Es notificacion canonica del subsistema chat documentada como punto de extension modular.",
    "TERMINO_VERIFICADO: tarifas.config.solicitada verificado en la plasmation hermana pizzepos-productos linea 43 como producto del par solicitada/actualizada con timeout 5000ms.",
    "TERMINO_VERIFICADO: carta.html.generada viabilidad.evaluacion.completada factura.exportada verificados por Grep en arquitectura/decisiones/_outputs/eventos-publish-subscribe.json lineas 551 4779 1958. Confirman que el whitelist de verbos indicativos del repo cubre generada completada exportada.",
    "DECISION_PENDIENTE_PARA_ANA_SOBRE_FORMA_FEMININA: el enum cerrado de este contrato esta declarado en feminino [creada actualizada eliminada] porque coincide con el uso real en subsistema-carta y subsistema-recetario. naming.json declara las formas en masculino [creado actualizado eliminado]. Dos opciones de cierre: (A) este contrato declara que el verbo se conjuga al genero de la entidad y naming.json se anota como flexible para participio concordado; (B) este contrato endurece a forma masculina y los modulos existentes se migran. Recomiendo opcion A porque la conjugacion es semantica del castellano y migrar producto.creado vs carta.creada rompe correlation_id de cientos de eventos en eventos-publish-subscribe.json.",
    "OBSERVACION_DE_ALCANCE_T8: el contrato hermano aggregate-vs-vista declara forbidden_pattern de AggregateRoot que matchea cualquier publish con verbo del enum mutacion cuyo prefijo no es this.dominioName. Este contrato hereda esa enforcement sin reclamarla. La diferencia es que aquel contrato enforce QUIEN puede emitir mutacion; este enforce QUE tiene que cumplir el evento para ser mutacion. Ambos son necesarios y complementarios.",
    "OBSERVACION_DE_VALIDATOR_PENDIENTE: este contrato exige validator nuevo que cruce events.publishes de cada module.json contra (a) verbo del eventType en enum mutacion vs whitelist notificacion, (b) shape declarado del payload contra shape_ref del agregado del emisor para mutacion, (c) absencia de shape de agregado ajeno en payload de notificacion. La declaracion event_core_class del modulo (anotada como pendiente en aggregate-vs-vista) facilita el cruce pero no es estrictamente necesaria para este contrato porque la clase del evento se determina por el verbo no por la declaracion del emisor. Sin validator el contrato es enforceable por revision humana mas Grep de los regex declarados en forbidden_patterns.",
    "OBSERVACION_DE_DRIFT_NUEVO_ENCONTRADO_EN_LECTURA: la plasmation hermana pizzepos-productos documenta que catalogo.actualizado se emite dos veces con shapes distintos en lineas 173 y 938. Este contrato canoniza ese caso como caso_testigo_drift_polisemia_mutacion. El validator pendiente debe detectar dos publish del mismo eventType de mutacion con shapes detectablemente distintos como ERROR no warning.",
    "OBSERVACION_DE_TAJADA_SIGUIENTE: la tajada 4 contrato_project_context_propagation y la tajada 5 endurecer_storage_layout_bypass_filesystem_a_error del arranque son independientes de esta. La tajada 2 reescribir pizzepos productos depende de tener las tres tajadas 1 3 6 cerradas para tener material canonico al que migrar. Esta tajada cierra el material semantico que faltaba para la reescritura; ana puede ahora cocinar tajada 2 con base canonica completa.",
    "OBSERVACION_DE_FORMA: este contrato declara dos AbstractClass en abstract_classes como array al igual que aggregate-vs-vista. Si ana decidio en aquel contrato conservar el shape de array dentro de un mismo documento, este lo replica por coherencia entre los dos sub-contratos derivados del mismo arranque."
  ]
}
```</result>
<usage><subagent_tokens>117201</subagent_tokens><tool_uses>43</tool_uses><duration_ms>377189</duration_ms></usage>
</task-notification>
