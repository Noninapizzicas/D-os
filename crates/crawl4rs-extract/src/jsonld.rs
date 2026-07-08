//! Extracción de JSON-LD / schema.org.
//!
//! Recoge los `<script type="application/ld+json">` de una página y devuelve
//! los objetos que contienen (aplanando arrays y `@graph`). Es la vía directa
//! a fichas de producto (`Product`: name, price, image, sku, availability) sin
//! adivinar selectores.

use scraper::{Html, Selector};
use serde_json::Value;

/// Extrae todos los objetos JSON-LD embebidos en el HTML.
pub fn extract_jsonld(html: &str) -> Vec<Value> {
    let doc = Html::parse_document(html);
    let selector =
        Selector::parse(r#"script[type="application/ld+json"]"#).expect("selector estático válido");

    let mut out = Vec::new();
    for el in doc.select(&selector) {
        let raw = el.text().collect::<String>();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
            flatten_into(value, &mut out);
        }
    }
    out
}

/// Aplana arrays y `@graph` en objetos individuales.
fn flatten_into(value: Value, out: &mut Vec<Value>) {
    match value {
        Value::Array(items) => {
            for item in items {
                flatten_into(item, out);
            }
        }
        Value::Object(ref obj) if obj.contains_key("@graph") => {
            if let Some(Value::Array(graph)) = obj.get("@graph") {
                for item in graph {
                    out.push(item.clone());
                }
            } else {
                out.push(value);
            }
        }
        other => out.push(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrae_product_de_una_ficha() {
        let html = r#"<html><head>
            <script type="application/ld+json">
            {"@context":"https://schema.org","@type":"Product",
             "name":"Teclado","sku":"TK-1","offers":{"@type":"Offer","price":"89.99"}}
            </script></head><body>x</body></html>"#;
        let objs = extract_jsonld(html);
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0]["@type"], "Product");
        assert_eq!(objs[0]["name"], "Teclado");
        assert_eq!(objs[0]["offers"]["price"], "89.99");
    }

    #[test]
    fn aplana_graph_y_arrays() {
        let html = r#"<script type="application/ld+json">
            {"@graph":[{"@type":"Organization","name":"Tienda"},
                       {"@type":"WebSite","name":"tienda.test"}]}
            </script>
            <script type="application/ld+json">
            [{"@type":"BreadcrumbList"},{"@type":"Product","name":"X"}]
            </script>"#;
        let objs = extract_jsonld(html);
        assert_eq!(objs.len(), 4);
        assert!(objs.iter().any(|o| o["@type"] == "Product"));
        assert!(objs.iter().any(|o| o["@type"] == "Organization"));
    }

    #[test]
    fn json_ld_malformado_se_ignora_sin_panic() {
        let html = r#"<script type="application/ld+json">{ roto, </script>"#;
        assert!(extract_jsonld(html).is_empty());
    }
}
