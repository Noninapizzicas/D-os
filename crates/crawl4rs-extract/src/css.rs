//! Extracción por selectores CSS.

use async_trait::async_trait;
use scraper::{Html, Selector};
use serde_json::{Map, Value};

use crate::{ExtractError, ExtractionStrategy, Result};

/// Especificación de un campo a extraer.
#[derive(Debug, Clone)]
pub struct FieldSpec {
    /// Nombre del campo en el JSON de salida.
    pub name: String,
    /// Selector CSS que localiza el elemento.
    pub selector: String,
    /// Atributo a leer; `None` → texto del elemento.
    pub attr: Option<String>,
    /// Si es `true`, recoge todas las coincidencias en un array.
    pub many: bool,
}

impl FieldSpec {
    /// Campo simple que extrae el texto de la primera coincidencia.
    pub fn text(name: impl Into<String>, selector: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selector: selector.into(),
            attr: None,
            many: false,
        }
    }

    /// Campo que extrae un atributo.
    pub fn attr(
        name: impl Into<String>,
        selector: impl Into<String>,
        attr: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            selector: selector.into(),
            attr: Some(attr.into()),
            many: false,
        }
    }

    /// Marca el campo como colección (array de todas las coincidencias).
    pub fn all(mut self) -> Self {
        self.many = true;
        self
    }
}

/// Extrae campos de un documento HTML según una lista de [`FieldSpec`].
#[derive(Debug, Clone)]
pub struct CssSelectorStrategy {
    fields: Vec<FieldSpec>,
}

impl CssSelectorStrategy {
    /// Crea la estrategia a partir de las especificaciones de campo.
    pub fn new(fields: Vec<FieldSpec>) -> Self {
        Self { fields }
    }
}

fn value_of(el: scraper::ElementRef, attr: &Option<String>) -> Value {
    match attr {
        Some(a) => Value::String(el.value().attr(a).unwrap_or_default().to_string()),
        None => {
            let text = el.text().collect::<String>();
            Value::String(text.split_whitespace().collect::<Vec<_>>().join(" "))
        }
    }
}

#[async_trait]
impl ExtractionStrategy for CssSelectorStrategy {
    fn name(&self) -> &'static str {
        "css"
    }

    async fn extract(&self, html: &str, _markdown: &str) -> Result<Value> {
        let doc = Html::parse_document(html);
        let mut obj = Map::new();

        for field in &self.fields {
            let selector = Selector::parse(&field.selector)
                .map_err(|_| ExtractError::BadSelector(field.selector.clone()))?;

            if field.many {
                let arr: Vec<Value> = doc
                    .select(&selector)
                    .map(|el| value_of(el, &field.attr))
                    .collect();
                obj.insert(field.name.clone(), Value::Array(arr));
            } else {
                let val = doc
                    .select(&selector)
                    .next()
                    .map(|el| value_of(el, &field.attr))
                    .unwrap_or(Value::Null);
                obj.insert(field.name.clone(), val);
            }
        }

        Ok(Value::Object(obj))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn extrae_titulo_y_precio() {
        let html = r#"<div class="card">
            <h2 class="t">Teclado mecánico</h2>
            <span class="price">49.99</span>
            <a class="link" href="/p/1">ver</a>
        </div>"#;
        let strat = CssSelectorStrategy::new(vec![
            FieldSpec::text("titulo", ".t"),
            FieldSpec::text("precio", ".price"),
            FieldSpec::attr("url", ".link", "href"),
        ]);
        let out = strat.extract(html, "").await.unwrap();
        assert_eq!(out["titulo"], "Teclado mecánico");
        assert_eq!(out["precio"], "49.99");
        assert_eq!(out["url"], "/p/1");
    }
}
