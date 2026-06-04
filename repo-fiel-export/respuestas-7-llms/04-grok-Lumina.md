# grok Lumina

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.

{
  "lenguaje": "Lumina",
  "descripcion": "Lenguaje híbrido diseñado específicamente para LLMs que combina JSON, Pseudocódigo y OOP de forma natural y eficiente.",
  "objetivo": "Permitir expresiones estructuradas, legibles por humanos y máquinas, con semántica orientada a objetos y flujo algorítmico claro.",
  "version": "0.1",

  "sintaxis_principal": {
    "extiende": ["JSON", "Pseudocode", "OOP"],
    "formato_base": "JSON enriquecido con bloques de pseudocódigo y declaraciones orientadas a objetos"
  },

  "prompt_sistema": "Eres un experto en Lumina, un lenguaje de programación híbrido para LLMs.\n\nReglas estrictas de Lumina:\n\n1. Toda respuesta debe ser un objeto JSON válido.\n2. Puedes usar bloques de pseudocódigo dentro de propiedades que empiecen por `code_` o `algo_`.\n3. Usa notación OOP con las claves: `class`, `extends`, `properties`, `methods`, `constructor`.\n4. Combina libremente JSON + pseudocódigo + OOP cuando sea más claro.\n5. Mantén siempre la respuesta dentro de un JSON válido.\n6. Sé extremadamente legible y estructurado.\n\nEjemplo de estructura recomendada:\n{\n  \"class\": \"NombreClase\",\n  \"extends\": \"ClasePadre\",\n  \"properties\": { ... },\n  \"methods\": {\n    \"nombreMetodo\": {\n      \"params\": [...],\n      \"code\": \"pseudocódigo aquí...\"\n    }\n  },\n  \"description\": \"...\",\n  \"data\": { ... }\n}\n\nResponde siempre usando Lumina.",

  "ejemplo_completo": {
    "class": "AgenteInteligente",
    "extends": "BaseLLMAgent",
    "properties": {
      "nombre": "string",
      "version": "0.1",
      "capacidades": ["razonamiento", "planificacion", "memoria"]
    },
    "constructor": {
      "params": ["nombre", "capacidades"],
      "code": "inicializar memoria\ncargar capacidades"
    },
    "methods": {
      "pensar": {
        "params": ["tarea"],
        "returns": "Plan",
        "code": "descomponer tarea en subtareas\npriorizar por impacto\nestimar recursos"
      },
      "ejecutar": {
        "params": ["plan"],
        "code": "for cada paso en plan:\n    si paso es JSON → parsear\n    si paso es acción → ejecutar"
      }
    },
    "data": {
      "estado_actual": "activo",
      "ultima_interaccion": "2026-06-03"
    }
  },

  "reglas_adicionales": [
    "Los bloques de código deben ir en strings multilínea dentro de propiedades `code_*`",
    "Se permite mezclar JSON normal con objetos Lumina",
    "Las clases pueden contener otras clases (composición)",
    "Se recomienda usar `steps` o `workflow` para procesos secuenciales"
  ],

  "proxima_version": "En v0.2 se añadirá soporte nativo para grafos de conocimiento y memoria vectorial dentro del mismo formato."
}grok
