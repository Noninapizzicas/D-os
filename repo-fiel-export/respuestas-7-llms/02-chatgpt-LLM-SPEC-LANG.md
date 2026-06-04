# chatgpt LLM SPEC LANG

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.

{
  "nombre": "LLM-SPEC-LANG",
  "objetivo": "Crear las bases de un lenguaje especial para trabajar con LLM, combinando JSON, pseudocódigo y programación orientada a objetos.",
  "prompt_maestro": {
    "rol": "Actúa como arquitecto de lenguajes, experto en JSON, pseudocódigo, programación orientada a objetos, diseño de DSL y sistemas para LLM.",
    "tarea": "Diseña desde cero un lenguaje especial pensado para que los LLM puedan interpretar, generar, validar y ejecutar razonamientos estructurados.",
    "contexto": {
      "idea_base": "El lenguaje debe englobar JSON, pseudocódigo y OOP.",
      "uso_principal": "Definir agentes, funciones, clases, flujos, reglas, herramientas, memoria, decisiones y ejecución paso a paso.",
      "prioridad": "Claridad, estructura, facilidad de parseo, extensibilidad y compatibilidad con automatización."
    },
    "requisitos_del_lenguaje": {
      "formato_base": "JSON válido o JSON-like estricto",
      "debe_soportar": [
        "objetos",
        "clases",
        "métodos",
        "funciones",
        "variables",
        "tipos",
        "condiciones",
        "bucles",
        "eventos",
        "acciones",
        "herramientas",
        "agentes",
        "memoria",
        "errores",
        "validaciones",
        "pipelines",
        "pseudocódigo legible"
      ],
      "debe_evitar": [
        "ambigüedad",
        "sintaxis excesivamente compleja",
        "dependencia de un lenguaje concreto",
        "mezclar texto libre sin estructura"
      ]
    },
    "estructura_esperada_de_salida": {
      "1_nombre_del_lenguaje": "Propón nombre corto y claro.",
      "2_filosofia": "Explica qué problema resuelve.",
      "3_sintaxis_base": "Define cómo se escribe.",
      "4_tipos_de_datos": "Define string, number, bool, list, object, null, ref, action, tool, agent.",
      "5_objetos_y_clases": "Diseña una forma de declarar clases, propiedades y métodos.",
      "6_funciones": "Define cómo se declaran funciones con input, output, steps y return.",
      "7_pseudocodigo": "Integra pseudocódigo dentro de bloques estructurados.",
      "8_control_de_flujo": "Define if, switch, loop, foreach, while y try/catch.",
      "9_agentes_llm": "Define agentes con rol, objetivo, herramientas, memoria y reglas.",
      "10_herramientas": "Define cómo invocar APIs, funciones externas o comandos.",
      "11_memoria": "Define memoria temporal, persistente y contextual.",
      "12_validacion": "Define reglas de validación del propio lenguaje.",
      "13_ejemplos": "Incluye ejemplos prácticos completos.",
      "14_parser": "Propón cómo construir un parser básico.",
      "15_versionado": "Define sistema de versiones del lenguaje."
    },
    "formato_de_respuesta": {
      "obligatorio": "Devuelve todo en JSON válido.",
      "sin_markdown": true,
      "sin_explicaciones_fuera_del_json": true,
      "idioma": "español"
    }
  },
  "ejemplo_minimo_del_lenguaje": {
    "lang": "LLM-SPEC-LANG",
    "version": "0.1",
    "agent": {
      "name": "Analizador",
      "role": "Analizar datos y tomar decisiones",
      "memory": {
        "type": "contextual",
        "fields": ["objetivo", "estado", "resultado"]
      },
      "class": {
        "name": "Producto",
        "properties": {
          "nombre": "string",
          "precio": "number",
          "stock": "number"
        },
        "methods": {
          "calcularMargen": {
            "input": {
              "coste": "number",
              "precioVenta": "number"
            },
            "steps": [
              "margen = precioVenta - coste",
              "porcentaje = margen / precioVenta * 100"
            ],
            "return": "porcentaje"
          }
        }
      },
      "flow": [
        {
          "step": 1,
          "action": "leer_producto"
        },
        {
          "step": 2,
          "if": "producto.stock < 5",
          "then": "avisar_reposicion",
          "else": "continuar"
        }
      ]
    }
  }
}chat GPT 
