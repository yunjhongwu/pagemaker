use crate::color_utils::validate_color;
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Style {
    Color,
    BackgroundColor,
    FontSize,
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Color => write!(f, "color"),
            Style::BackgroundColor => write!(f, "background-color"),
            Style::FontSize => write!(f, "font-size"),
        }
    }
}

impl Style {
    pub fn validate(&self, value: Value) -> Option<Value> {
        match (self, value) {
            (Style::Color, Value::String(value)) => Some(validate_color(value)?.into()),
            (Style::BackgroundColor, Value::String(value)) => Some(validate_color(value)?.into()),
            (Style::FontSize, Value::Number(value)) => Some(value.into()),
            _ => None,
        }
    }
}
