# deepseek LINC

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.

{
  "lenguaje": "LINC (Linguistic Intermediate Notation for Cognition)",
  "objetivo": "Un lenguaje intermedio para LLMs que unifica JSON, seudocódigo y principios de programación orientada a objetos (OPP), facilitando la generación, razonamiento y ejecución simbólica.",
  "prompt_para_llm": {
    "rol": "Eres un arquitecto de lenguajes especializado en representación del conocimiento para IA. Debes diseñar LINC, un lenguaje que integre JSON (estructura de datos), seudocódigo (algoritmos legibles) y OOP (encapsulación, herencia, polimorfismo).",
    "instrucciones": [
      "Define la sintaxis básica unificada: usa llaves {} para objetos (como JSON), pero permite métodos y herencia.",
      "Los tipos de datos serán: number, string, boolean, null, array, object (con clave:valor), y func (para métodos).",
      "Las operaciones se escriben en seudocódigo estilo natural pero estructurado: 'if condicion then ... else ...', 'for each item in lista', 'while condicion', 'func nombre(parametros) -> retorno'.",
      "Soporta herencia con 'extiende' y polimorfismo por sobrescritura de métodos.",
      "Permite decoradores como '@doc' para documentación y '@validar' para reglas.",
      "Incluye bloques 'entender' para razonamiento del LLM y 'generar' para producir salida.",
      "Asegura que cualquier fragmento LINC pueda traducirse a JSON puro + pseudocódigo ejecutable en Python/JS."
    ],
    "ejemplo_de_uso": {
      "descripcion": "Define un objeto 'Persona' con método 'saludar', y un objeto 'Empleado' que lo extiende.",
      "codigo_linc": {
        "objeto": "Persona",
        "propiedades": {
          "nombre": "string",
          "edad": "number"
        },
        "metodos": {
          "saludar": "func (otro_nombre) -> string { retorna 'Hola, ' + otro_nombre + ', soy ' + this.nombre }"
        },
        "doc": "@doc('Representa una persona básica')"
      },
      "herencia": {
        "objeto": "Empleado",
        "extiende": "Persona",
        "propiedades_adicionales": {
          "salario": "number",
          "puesto": "string"
        },
        "metodos_adicionales": {
          "trabajar": "func () -> string { retorna this.nombre + ' está trabajando como ' + this.puesto }"
        },
        "sobrescritura": {
          "saludar": "func (otro_nombre) -> string { retorna 'Laboralmente: ' + super.saludar(otro_nombre) }"
        }
      },
      "ejecucion_en_pseudocodigo": {
        "inicio": "empleado = crear Empleado(nombre='Ana', edad=30, salario=50000, puesto='Ingeniera')",
        "mostrar": "escribir empleado.saludar('Carlos')",
        "validacion": "@validar(empleado.salario > 0)",
        "salida_esperada": "'Laboralmente: Hola, Carlos, soy Ana'"
      }
    },
    "reglas_adicionales": [
      "Todo LINC debe poderse serializar a JSON sin pérdida, añadiendo un campo '_tipo' para objetos.",
      "El seudocódigo debe ser suficientemente claro para que un LLM lo ejecute simbólicamente sin intérprete real.",
      "Soporta comentarios con '//' y bloques de documentación con '/* ... */'."
    ],
    "formato_de_respuesta_final": "Devuelve el diseño completo de LINC en un objeto JSON que incluya: nombre, version, sintaxis_BFC, ejemplo_integral, y gramatica_resumida en formato EBNF."
  },
  "mi_respuesta_como_asistente": "He generado el prompt anterior para que, al enviarlo a un LLM, éste desarrolle las bases completas de LINC. Si deseas que yo mismo desarrolle el lenguaje sin enviar el prompt, indícalo y lo haré directamente en el siguiente mensaje."
} deepseek 
