use crate::Style;
use serde_json::{Map, Value};

#[derive(Debug, Default, Clone)]
pub struct Config {
    value: Value,
}

impl Config {
    pub fn new() -> Self {
        Self {
            value: Map::new().into(),
        }
    }

    pub fn set_style(&mut self, style: Style, value: Value) -> &Self {
        let _ = style
            .validate(value)
            .map(|v| self.value[style.to_string()] = v);

        self
    }

    pub fn get_style(&self) -> String {
        if let Some(map) = self.value.as_object() {
            if !map.is_empty() {
                return format!(
                    "style=\"{}\"",
                    map.iter()
                        .map(|(k, v)| match v {
                            Value::String(v) => format!("{}:{}", k, v.as_str()),
                            Value::Number(v) => format!("{}:{}", k, v),
                            _ => String::default(),
                        })
                        .collect::<Vec<_>>()
                        .join(";")
                );
            }
        }

        String::default()
    }
}
