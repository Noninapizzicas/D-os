# Fiel + notario — experimento de lenguaje formal para LLMs en event-core

Repositorio extraído del experimento desarrollado en `noninapizzicas/2enki` durante junio 2026. Contiene un lenguaje formal JSON+OOP+pseudocódigo (**Fiel v0.1.0**) diseñado para canonizar contratos arquitectónicos de sistemas event-core, junto con el agente Claude Code (**notario**) que sólo escribe en ese lenguaje, y todo el material empírico que originó ambos.

## Por qué existe este repo

El experimento empezó al detectar que **el LLM escribe contracorriente de su corpus aprendido cuando opera en paradigma event-core**. Cada LLM (Claude incluido) trae tendencias por defecto (cache defensiva, fallback silencioso, bypass de filesystem, catch tragador, etc.) que producen drift sistemático en módulos event-core. La cura: definir un lenguaje formal con conciencia inyectada + un agente que sólo habla ese lenguaje y verifica términos contra el repo antes de plasmar.

## Estructura del repo

```
.
├── README.md                          (este archivo)
├── CLAUDE.md                          (índice maestro del repo 2enki origen)
│
├── lenguaje/                          el lenguaje formal Fiel v0.1.0
│   ├── _fiel-v0.1.0.json              síntesis final del experimento 7-LLMs
│   ├── _experimento-3coops-001.json   sustrato 3coops = code + OOP + pseudo-code
│   └── _prompt-disenyo-lenguaje-3coops-v0.1.json   prompt usado en el experimento
│
├── agente-notario/                    el agente Claude Code que sólo habla Fiel
│   ├── notario.md                     wrapper mínimo Claude Code
│   └── notario.json                   shape operativo escrito en el propio Fiel
│                                      (self-bootstrap)
│
├── cocinado/                          el análisis origen
│   └── _arranque-modulo-event-core-disciplina.json
│                                      8 tendencias T1-T8 + insight centralismo_local
│
├── plasmaciones/                      7 documentos producidos por notario
│   ├── _contrato-modulo-event-core-disciplina-via-notario.json
│   ├── _pizzepos-productos-disciplina-via-notario.json
│   ├── _contrato-aggregate-vs-vista-via-notario.json
│   ├── _contrato-project-context-propagation-via-notario.json
│   ├── _propuesta-endurecer-storage-layout-bypass-via-notario.json
│   ├── _contrato-evento-mutacion-vs-notificacion-via-notario.json
│   └── _tendencias-llm-en-event-core-via-notario.json   ← catálogo canónico T1-T8
│
├── contratos-referenciados/           contratos del repo 2enki que el experimento cita
│   ├── events.contract.json
│   ├── errors.contract.json
│   ├── persistence.contract.json
│   ├── naming.json
│   ├── project-identity.contract.json
│   ├── dinamica-de-trabajo-companero.contract.json
│   ├── disciplina-llm-operador.contract.json
│   ├── paradigma-no-cabe.contract.json
│   └── storage-layout.contract.json
│
├── respuestas-7-llms/                 material empírico del experimento
│   ├── 01-kimi-StructoLang.md
│   ├── 02-chatgpt-LLM-SPEC-LANG.md
│   ├── 03-deepseek-LINC.md
│   ├── 04-grok-Lumina.md
│   ├── 05-manus-CognitoScript.md
│   ├── 06-gemini-LOP.md
│   ├── 07-claude-puro-Fiel.md         ← base de la síntesis final
│   ├── 08-kimi-ContraForma.md         (segunda ronda con prompt mejorado)
│   ├── 09-deepseek-CanonLang.md       (segunda ronda)
│   └── 11-gemini-KanonJSON.md         (segunda ronda)
│                                      (falta 10-grok-CanonLang-v2: no se localizó
│                                       en el transcript, posiblemente no se pegó)
│
├── skills-hermanas/                   contexto del pipeline ana → notario → fede
│   ├── ana-SKILL.md                   cocina horizontes abiertos (conversa con humano)
│   └── fede-SKILL.md                  ejecuta horizontes cerrados (implementa código)
│
└── conversacion/                      la sesión completa que originó todo
    ├── sesion-dios-COMPLETA.txt       legible (596 KB, 383 turnos extraídos)
    └── sesion-dios-COMPLETA.jsonl     crudo formato Claude Code (9.3 MB)
```

## Cronología del experimento

1. **2026-06-02** — Conversación arranca con "compañero necesito una mente analítica". Pivote hacia "Claude como dios del sistema" → diagnostica + propone + no aplica.
2. **Auditoría en runtime** del módulo `pizzepos/productos` en proyecto real (Vapers): detección de 5 tendencias simultáneas (T1, T2, T3, T5, T7).
3. **Análisis cocinado**: 8 tendencias T1-T8 con casos testigo archivo:línea.
4. **Diseño del prompt** con conciencia inyectada (las 8 tendencias como vacuna).
5. **Experimento empírico**: el prompt pasado a 7 LLMs (Kimi, ChatGPT, DeepSeek, Grok, Manus, Gemini, Claude puro nueva sesión). Las 6 versiones omitieron estructuralmente el modelado de tendencias — la 7ª (Claude puro con el prompt) la integró completa.
6. **Síntesis Fiel v0.1.0**: Fiel (Claude puro) como base + 4 piezas aditivas (`enforcedTendencies` por clase, `severidad` enum, `auto_audit` con detectores extra, `BusConnection` primitiva nativa).
7. **Agente notario** como subagente Claude Code (self-bootstrap: se define a sí mismo en Fiel).
8. **Primera prueba**: notario plasma el contrato del padre `ModuloEventCore`. Detecta T8 en runtime — corrige términos inventados (`CacheIncremental`, `_withCode`) contra el repo.
9. **Sexteto de tajadas**: notario plasma 6 contratos derivados (aggregate-vs-vista, project-context-propagation, mutación-vs-notificación, etc.). Destapa 4 driftes nuevos no documentados.
10. **Catálogo canónico** de las 8 tendencias como fuente autoritativa única.

## Lectura recomendada por orden

Si quieres entender el experimento de principio a fin:

1. `conversacion/sesion-dios-COMPLETA.txt` — la historia narrada.
2. `cocinado/_arranque-modulo-event-core-disciplina.json` — el análisis origen.
3. `lenguaje/_prompt-disenyo-lenguaje-3coops-v0.1.json` — el prompt del experimento.
4. `respuestas-7-llms/*.md` — qué produjo cada LLM en frío.
5. `lenguaje/_fiel-v0.1.0.json` — la síntesis final.
6. `agente-notario/notario.json` — el agente self-bootstrap en Fiel.
7. `plasmaciones/_tendencias-llm-en-event-core-via-notario.json` — el catálogo canónico.
8. `plasmaciones/*.json` (resto) — el sexteto en acción.

## El pipeline ana / notario / fede

Tres piezas complementarias de Claude Code:

- **ana** (skill): personalidad que cocina horizontes abiertos en conversación con el humano. Escucha antes de cerrar.
- **notario** (agente): plasma análisis cocinado en Fiel. NO conversa. Verifica términos. Devuelve UN JSON puro.
- **fede** (skill): ejecuta horizontes cerrados. Carga plan + ejecutor, declara axiomas, ejecuta con OK explícito.

El testigo viaja entre los tres. Cada uno tiene su scope estricto.

## Estado del experimento

**Fiel v0.1.0 y notario**: funcionando en producción en `noninapizzicas/2enki`. Notario plasmó 8 documentos durante la sesión documentada. Detectó driftes que ningún validator captura. Se autocorrigió cuando los términos del insumo no existían en el repo.

**Pendiente**:
- Validator AJV de Fiel (self-bootstrap mecánico, hoy declarado pero no enforced).
- 4 decisiones que notario delegó a ana (modelado de dominio en `pizzepos/productos`, forma de verbo masculino vs femenino, validator nuevo vs extender existente, ampliación whitelist filesystem).
- Decidir si Fiel asciende a `_contratos/` del repo 2enki o queda como experimento separado.

## Licencia y contexto

Material extraído de un repo privado de trabajo. La estructura conceptual (lenguaje + agente + experimento) es replicable a cualquier sistema event-core que requiera canonizar disciplina arquitectónica frente a las tendencias del LLM al escribir código.
