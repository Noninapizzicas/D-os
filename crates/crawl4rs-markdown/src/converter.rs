//! Conversión HTML → Markdown.
//!
//! Implementación propia, ligera, inspirada en `turndown.js`. Recorre el DOM
//! (vía `scraper`/`ego-tree`) y emite Markdown para el subconjunto de
//! etiquetas relevante en contenido de artículos. No pretende cubrir todo
//! HTML; cubre lo que importa para producir texto legible por un LLM.

use ego_tree::NodeRef;
use scraper::{Html, Node};

use crate::cleaner;

/// Resultado de convertir HTML a Markdown.
pub struct Converted {
    /// Markdown resultante.
    pub markdown: String,
    /// URLs (href) encontradas, en orden de aparición.
    pub links: Vec<String>,
}

/// Convierte un documento HTML completo a Markdown.
pub fn html_to_markdown(html: &str) -> Converted {
    let document = Html::parse_document(html);
    let mut ctx = Ctx::default();
    walk(document.tree.root(), &mut ctx);
    Converted {
        markdown: normalize(&ctx.out),
        links: ctx.links,
    }
}

#[derive(Default)]
struct Ctx {
    out: String,
    links: Vec<String>,
    list_depth: usize,
    /// Contador de ítems para listas ordenadas, por nivel.
    ordered_counters: Vec<usize>,
}

fn walk(node: NodeRef<Node>, ctx: &mut Ctx) {
    match node.value() {
        Node::Text(text) => {
            ctx.out.push_str(&collapse_ws(text));
        }
        Node::Element(el) => {
            let tag = el.name();
            if cleaner::is_stripped(tag) || cleaner::is_noise(tag) {
                return;
            }
            render_element(node, tag, el, ctx);
        }
        // Document, Fragment, Comment, Doctype, ProcessingInstruction: sólo se
        // desciende a los hijos.
        _ => walk_children(node, ctx),
    }
}

fn walk_children(node: NodeRef<Node>, ctx: &mut Ctx) {
    for child in node.children() {
        walk(child, ctx);
    }
}

fn render_element(node: NodeRef<Node>, tag: &str, el: &scraper::node::Element, ctx: &mut Ctx) {
    match tag {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let level = tag[1..].parse::<usize>().unwrap_or(1);
            block_gap(ctx);
            ctx.out.push_str(&"#".repeat(level));
            ctx.out.push(' ');
            walk_children(node, ctx);
            block_gap(ctx);
        }
        "p" | "div" | "section" | "article" | "main" | "header" => {
            block_gap(ctx);
            walk_children(node, ctx);
            block_gap(ctx);
        }
        "br" => ctx.out.push('\n'),
        "hr" => {
            block_gap(ctx);
            ctx.out.push_str("---");
            block_gap(ctx);
        }
        "strong" | "b" => wrap(node, ctx, "**", "**"),
        "em" | "i" => wrap(node, ctx, "*", "*"),
        "code" => {
            // `code` dentro de `pre` se maneja en la rama `pre`.
            if !inside_pre(node) {
                wrap(node, ctx, "`", "`");
            } else {
                walk_children(node, ctx);
            }
        }
        "pre" => {
            block_gap(ctx);
            ctx.out.push_str("```\n");
            let text = text_content(node);
            ctx.out.push_str(text.trim_end());
            ctx.out.push_str("\n```");
            block_gap(ctx);
        }
        "blockquote" => {
            block_gap(ctx);
            let mut inner = Ctx::default();
            walk_children(node, &mut inner);
            for line in normalize(&inner.out).lines() {
                ctx.out.push_str("> ");
                ctx.out.push_str(line);
                ctx.out.push('\n');
            }
            ctx.links.append(&mut inner.links);
            block_gap(ctx);
        }
        "ul" => render_list(node, ctx, false),
        "ol" => render_list(node, ctx, true),
        "li" => render_list_item(node, ctx),
        "a" => render_anchor(node, el, ctx),
        "img" => render_img(el, ctx),
        // Etiquetas en línea o desconocidas: descender sin decorar.
        _ => walk_children(node, ctx),
    }
}

fn render_list(node: NodeRef<Node>, ctx: &mut Ctx, ordered: bool) {
    block_gap(ctx);
    ctx.list_depth += 1;
    ctx.ordered_counters.push(if ordered { 1 } else { 0 });
    walk_children(node, ctx);
    ctx.ordered_counters.pop();
    ctx.list_depth -= 1;
    block_gap(ctx);
}

fn render_list_item(node: NodeRef<Node>, ctx: &mut Ctx) {
    let depth = ctx.list_depth.saturating_sub(1);
    let indent = "  ".repeat(depth);
    ctx.out.push('\n');
    ctx.out.push_str(&indent);

    let ordered = ctx
        .ordered_counters
        .last()
        .copied()
        .map(|c| c > 0)
        .unwrap_or(false);
    if ordered {
        let n = ctx.ordered_counters.last().copied().unwrap_or(1);
        ctx.out.push_str(&format!("{n}. "));
        if let Some(c) = ctx.ordered_counters.last_mut() {
            *c += 1;
        }
    } else {
        ctx.out.push_str("- ");
    }

    let start = ctx.out.len();
    walk_children(node, ctx);
    // Colapsa saltos de línea internos del ítem a espacios.
    let item: String = ctx.out.split_off(start).replace('\n', " ");
    ctx.out.push_str(item.trim_end());
}

fn render_anchor(node: NodeRef<Node>, el: &scraper::node::Element, ctx: &mut Ctx) {
    let href = el.attr("href").unwrap_or_default().to_string();
    if !href.is_empty() {
        ctx.links.push(href.clone());
    }
    let text = text_content(node);
    let text = text.trim();
    if href.is_empty() || text.is_empty() {
        ctx.out.push_str(text);
    } else {
        ctx.out.push_str(&format!("[{text}]({href})"));
    }
}

fn render_img(el: &scraper::node::Element, ctx: &mut Ctx) {
    let src = el.attr("src").unwrap_or_default();
    if src.is_empty() {
        return;
    }
    let alt = el.attr("alt").unwrap_or_default();
    ctx.out.push_str(&format!("![{alt}]({src})"));
}

fn wrap(node: NodeRef<Node>, ctx: &mut Ctx, open: &str, close: &str) {
    let inner = text_content(node);
    if inner.trim().is_empty() {
        return;
    }
    ctx.out.push_str(open);
    walk_children(node, ctx);
    ctx.out.push_str(close);
}

/// Devuelve el texto plano concatenado bajo un nodo.
fn text_content(node: NodeRef<Node>) -> String {
    let mut s = String::new();
    for d in node.descendants() {
        if let Node::Text(t) = d.value() {
            s.push_str(&collapse_ws(t));
        }
    }
    s
}

fn inside_pre(node: NodeRef<Node>) -> bool {
    node.ancestors().any(|a| match a.value() {
        Node::Element(e) => e.name() == "pre",
        _ => false,
    })
}

/// Colapsa espacios en blanco consecutivos a un único espacio.
fn collapse_ws(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut prev_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out
}

/// Inserta un separador de bloque (línea en blanco) sin duplicarlo.
fn block_gap(ctx: &mut Ctx) {
    while ctx.out.ends_with(' ') {
        ctx.out.pop();
    }
    if !ctx.out.ends_with("\n\n") && !ctx.out.is_empty() {
        if ctx.out.ends_with('\n') {
            ctx.out.push('\n');
        } else {
            ctx.out.push_str("\n\n");
        }
    }
}

/// Normaliza el Markdown: recorta y colapsa líneas en blanco excesivas.
fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut blank_run = 0usize;
    for line in s.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            blank_run += 1;
            if blank_run <= 1 {
                out.push('\n');
            }
        } else {
            blank_run = 0;
            out.push_str(trimmed);
            out.push('\n');
        }
    }
    out.trim().to_string()
}
