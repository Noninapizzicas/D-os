# gemini KanonJSON

> Extraído del transcript de la sesión.
> Pegado en chat por el humano companyero, NO generado por Claude Code.

{
"meta": {
"id": "3coops-arch-lang-v0.1",
"version": "0.1.0",
"creado": "2026-06-03",
"language": "es-ES",
"supersedes": "7-LLMs-experiments-2026-06-02"
},
"name": "KanonJSON",
"philosophy_breve": "Formato estricto de canonizacion arquitectonica que encapsula logica, estructura y sesgos del LLM en contenedores JSON validables, eliminando la deriva cognitiva y la libre interpretacion del modelo.",
"componentes": [
{
"nombre": "SintaxisContenedora",
"tipo": "JSON",
"descripcion": "Estructura rigida que elimina cualquier variacion sintactica o formateo libre fuera del estandar ECMAScript JSON."
},
{
"nombre": "ModeladoConceptual",
"tipo": "OOP",
"descripcion": "Definicion de contratos arquitectonicos mediante clases abstratacas, propiedades tipadas, invariantes explicitas y patrones prohibidos."
},
{
"nombre": "LogicaAlgoritmica",
"tipo": "PseudocodigoEstructuradoShape",
"descripcion": "Sentencias logicas divididas en componentes atomicos mecanizables, evitando cadenas de texto planas multilinea."
}
],
"primitivas_canonicas": [
{
"name": "Class_Declaration",
"type": "object",
"shape_json": {
"type": "object",
"properties": {
"type": { "type": "string", "pattern": "^(abstract_class|class)$" },
"name": { "type": "string", "pattern": "^[A-Z][a-zA-Z0-9]$", "maxLength": 50 },
"extends": { "type": "string", "pattern": "^[A-Z][a-zA-Z0-9_]$", "maxLength": 50 },
"properties": { "type": "array", "items": { "$ref": "#/definitions/Property" } },
"constructor": { "ref": "#/definitions/Constructor" },
"methods": { "type": "array", "items": { "$ref": "#/definitions/Method" } },
"concrete_invariants": { "type": "array", "items": { "type": "string", "maxLength": 150 } },
"forbidden_patterns": { "type": "array", "items": { "type": "string", "maxLength": 100 } }
},
"required": ["type", "name", "properties", "methods", "concrete_invariants", "forbidden_patterns"]
}
},
{
"name": "Property",
"type": "object",
"shape_json": {
"type": "object",
"properties": {
"name": { "type": "string", "pattern": "^[a-z][a-zA-Z0-9_]*$", "maxLength": 40 },
"data_type": { "type": "string", "pattern": "^(string|number|boolean|object|array|bus_connection)", "maxLength": 20 },
"required": { "type": "boolean" }
},
"required": ["name", "data_type", "required"]
}
},
{
"name": "Constructor",
"type": "object",
"shape_json": {
"type": "object",
"properties": {
"params": { "type": "array", "items": { "ref": "#/definitions/Property" } },
"steps": { "type": "array", "items": { "$ref": "#/definitions/Step" } }
},
"required": ["params", "steps"]
}
},
{
"name": "Method",
"type": "object",
"shape_json": {
"type": "object",
"properties": {
"name": { "type": "string", "pattern": "^[a-z][a-zA-Z0-9_]*$", "maxLength": 40 },
"returns": { "type": "string", "maxLength": 30 },
"steps": { "type": "array", "items": { "$ref": "#/definitions/Step" } }
},
"required": ["name", "returns", "steps"]
}
},
{
"name": "Step",
"type": "object",
"shape_json": {
"type": "object",
"properties": {
"action": { "type": "string", "pattern": "^(assign|invoke_bus|evaluate_condition|throw_error|return_value)$", "maxLength": 30 },
"target": { "type": "string", "pattern": "^[a-zA-Z0-9_\\.\\'\"]+", "maxLength": 80 },
"condition": { "type": "string", "maxLength": 150 },
"value": { "type": "string", "maxLength": 150 }
},
"required": ["action", "target", "condition", "value"]
}
}
],
"AbstractClass_ejemplar_que_modela_caso_de_uso_real": {
"type": "abstract_class",
"name": "AbstractArchitecturalContract",
"extends": "None",
"properties": [
{ "name": "bus", "data_type": "bus_connection", "required": true },
{ "name": "contractId", "data_type": "string", "required": true }
],
"concrete_invariants": [
"Toda operacion de lectura o escritura de datos debe pasar obligatoriamente por el bus de eventos unificado.",
"Queda prohibida la persistencia local de estados o caches que dupliquen la verdad centralizada del bus."
],
"forbidden_patterns": [
"this.cache =",
"require('fs')",
"catch()"
],
"methods": [
{
"name": "validateArchitectureState",
"returns": "boolean",
"steps": [
{
"action": "invoke_bus",
"target": "this.bus.publishAndWait",
"condition": "None",
"value": "'architecture.validate.request', { id: this.contractId }"
},
{
"action": "evaluate_condition",
"target": "None",
"condition": "if (!result.isValid)",
"value": "throw ARCHITECTURE_DRIFT_DETECTED"
},
{
"action": "return_value",
"target": "None",
"condition": "None",
"value": "true"
}
]
}
]
},
"tendencias_del_llm": [
{
"id": "T1_cache_defensiva",
"creencia_aprendida_corpus": "Memorizar evita llamadas extra al bus y mejora performance",
"verdad_paradigma_que_la_contradice": "Bus es fuente unica de autoridad; memorizar duplica autoridad",
"patron_codigo_a_evitar_regex": "this\.?cache\s*=\snew Map|this\.\w+PerProject\s=\snew Map",
"alternativa_canonica_pseudocodigo": "result = await this.bus.publishAndWait(eventType, payload)",
"caso_testigo_repo_concreto": "modules/pizzepos/productos/index.js:28"
},
{
"id": "T2_fallback_silencioso_identidad",
"creencia_aprendida_corpus": "Si el id no resuelve busco algo parecido para no romper flujo",
"verdad_paradigma_que_la_contradice": "Id no resuelto es bug; fallar ruidoso expone el desencuentro",
"patron_codigo_a_evitar_regex": "resolveTo\wFallback|for.of this\.\w+\.keys.return",
"alternativa_canonica_pseudocodigo": "if (!cache.has(id)) throw INVALID_INPUT",
"caso_testigo_repo_concreto": "modules/pizzepos/productos/index.js:62"
},
{
"id": "T3_bypass_filesystem",
"creencia_aprendida_corpus": "fs.readdir y fs.readFile son APIs estandar; usarlas es eficiente",
"verdad_paradigma_que_la_contradice": "filesystem es modulo dueno del scope; bus es la unica forma de mantener autoridad",
"patron_codigo_a_evitar_regex": "require\(['"]fs['"]\)|await fs\.(readdir|readFile|writeFile)",
"alternativa_canonica_pseudocodigo": "result = await this.bus.publishAndWait('fs.read.request', payload)",
"caso_testigo_repo_concreto": "modules/pizzepos/productos/index.js:1133"
},
{
"id": "T4_catch_tragador",
"creencia_aprendida_corpus": "try catch vacio para que el flujo no rompa",
"verdad_paradigma_que_la_contradice": "Error oculto se manifiesta despues como inconsistencia inexplicable",
"patron_codigo_a_evitar_regex": "catch\s\(\s_\s*\)\s*\{\s*\}",
"alternativa_canonica_pseudocodigo": "catch (err) { logger.error('mod.op.failed', err); throw err }",
"caso_testigo_repo_concreto": "modules/filesystem/index.js:141"
},
{
"id": "T5_sobreescribir_estado_al_recargar",
"creencia_aprendida_corpus": "Al cargar normalizo campos como activos para tener estado consistente",
"verdad_paradigma_que_la_contradice": "Estado persistido es la verdad; sobreescribir en carga pierde el estado real",
"patron_codigo_a_evitar_regex": "\{\s*\.\.\.\w+,\sactivo:\strue\s*\}",
"alternativa_canonica_pseudocodigo": "const item = { ...rawData } // sin sobreescribir",
"caso_testigo_repo_concreto": "modules/pizzepos/productos/index.js:1167"
},
{
"id": "T6_completar_campos_que_pseudocodigo_no_pide",
"creencia_aprendida_corpus": "El campo esta vacio; pongo un valor razonable",
"verdad_paradigma_que_la_contradice": "pseudocodigo es ley; nowISO es nowISO; valores no especificados no se inventan",
"patron_codigo_a_evitar_regex": "created_at:\s*['"][0-9]{4}-|version:\s2\s,",
"alternativa_canonica_pseudocodigo": "created_at: nowISO(); version: 1",
"caso_testigo_repo_concreto": "audit_2026-06-02_carta_catalogo_activo"
},
{
"id": "T7_handoff_prematuro",
"creencia_aprendida_corpus": "Eventual consistency es legitimo; publico al empezar la accion",
"verdad_paradigma_que_la_contradice": "Publica cuando el cambio aterrizo o con payload completo para que consumers no necesiten leer disco",
"patron_codigo_a_evitar_regex": "this\.publish\([^)]+\);[\s\S]{0,200}?await.*write",
"alternativa_canonica_pseudocodigo": "await write.then(publish('foo.creada', payloadCompleto))",
"caso_testigo_repo_concreto": "audit_2026-06-02_flujo_carta_manager"
},
{
"id": "T8_deducir_terminos_nuevos_del_contexto_inmediato",
"creencia_aprendida_corpus": "Cuando el humano usa termino tecnico nuevo lo deduzco del contexto reciente",
"verdad_paradigma_que_la_contradice": "Deducir mal contamina el documento; preguntar es mas barato que reescribir",
"patron_codigo_a_evitar_regex": "// asumir significado sin verificar",
"alternativa_canonica_pseudocodigo": "if (termino_nuevo && !termino_existe_en_repo) return requires_clarification(termino_nuevo)",
"caso_testigo_repo_concreto": "arquitectura/decisiones/propuestas/_experimento-3coops-001.json"
}
],
"auto_audit_protocol": {
"name": "AutoAuditExecution",
"steps": [
{
"action": "evaluate_condition",
"target": "this.generatedOutput",
"condition": "if (this.generatedOutput.containsPattern(forbidden_patterns_meta))",
"value": "this.triggerRegeneration(current_section)"
},
{
"action": "evaluate_condition",
"target": "this.generatedOutput",
"condition": "if (this.generatedOutput.hasOmission(tendencias_del_llm))",
"value": "this.triggerRegeneration('tendencias_del_llm')"
},
{
"action": "return_value",
"target": "None",
"condition": "None",
"value": "this.finalizeOutput()"
}
]
},
"output_rules_para_LLM_que_use_el_lenguaje": [
"Regrada 1: El documento final debe ser estrictamente un objeto JSON unico y valido bajo la especificacion AJV sin texto previo ni posterior.",
"Regla 2: El pseudocodigo explicativo de metodos o procesos debe seguir estrictamente la estructura shape dividida por objetos.",
"Regla 3: No se permite la suposicion o inferencia automatica de parametros tecnologicos que no esten explicitamente descritos en los requisitos del modulo.",
"Regla 4: Se debe adjuntar siempre la declaracion de patrones prohibidos y el mapeo de las 8 tendencias criticas (T1-T8).",
"Regla 5: Se rechaza cualquier uso de Markdown, formateo enriquecido o comentarios nativos dentro o fuera de las propiedades string de JSON."
],
"forbidden_patterns_meta": [
"Uso de bloques de codigo triple comilla invertida para envolver fragmentos internos de logica",
"Definicion de metodos mediante strings planos multilinea en lugar del array de objetos shape",
"Mezcla de ingles y espanyol en nombres de llaves primitivas o valores de metadatos",
"Inclusion de explicaciones discursivas textuales largas que superen el maxLength de 300 caracteres"
],
"ejemplo_completo": {
"class_definition": {
"type": "class",
"name": "CatalogoValidatorModule",
"extends": "AbstractArchitecturalContract",
"properties": [
{ "name": "bus", "data_type": "bus_connection", "required": true },
{ "name": "contractId", "data_type": "string", "required": true }
],
"concrete_invariants": [
"El validador nunca usara variables locales persistentes para omitir lecturas del bus."
],
"forbidden_patterns": [
"this.cache = new Map()",
"require('fs')"
],
"methods": [
{
"name": "validarProductoActivo",
"returns": "object",
"steps": [
{
"action": "invoke_bus",
"target": "this.bus.publishAndWait",
"condition": "None",
"value": "'catalog.product.get', { id: 'prod_101' }"
},
{
"action": "evaluate_condition",
"target": "None",
"condition": "if (!result.item)",
"value": "throw INVALID_INPUT"
},
{
"action": "return_value",
"target": "None",
"condition": "None",
"value": "{ valid: result.item.activo }"
}
]
}
]
}
},
"evolution_roadmap": {
"v0_1_0_actual": "Definicion basica de la sintaxis JSON contenedora, las 8 tendencias estructuradas del LLM y pseudocodigo por objetos shape parametrizados.",
"v0_2_0_futuro": "Inclusion de validadores integrados basados en esquemas JSONSchema embebidos directos para propiedades dinamicas complejas sin romper tipado.",
"v0_3_0_futuro": "Modulo automatico de compilacion bidireccional entre interfaces de abstraccion nativa y arboles de sintaxis abstracta (AST)."
}
} géminis 
