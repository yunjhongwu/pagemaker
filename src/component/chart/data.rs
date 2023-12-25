use anyhow::Result;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Pair<XType: Display> {
    x: XType,
    y: f64,
}

impl<XType: Display + Serialize> Pair<XType> {
    pub fn new(x: XType, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_json(&self) -> Value {
        json!({ "x": self.x, "y": self.y })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Dataset<XType: Display + Serialize> {
    label: Option<String>,
    data: Vec<Pair<XType>>,
}

impl<XType: Display + Serialize> Dataset<XType> {
    pub fn new() -> Self {
        Self {
            label: None,
            data: Vec::new(),
        }
    }

    pub fn from_vec(data: Vec<Pair<XType>>) -> Self {
        Self { label: None, data }
    }

    pub fn from_zip(x: Vec<XType>, y: Vec<f64>) -> Option<Self> {
        if x.len() != y.len() {
            return None;
        }
        Some(Self {
            label: None,
            data: x.into_iter().zip(y).map(|(x, y)| Pair::new(x, y)).collect(),
        })
    }

    pub fn set_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());

        self
    }

    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn push(&mut self, x: XType, y: f64) {
        self.data.push(Pair::new(x, y));
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_json(&self) -> Result<Value> {
        let mut config = Map::new();
        if let Some(label) = &self.label {
            config.insert("label".to_string(), label.as_str().into());
        }

        let data = self.data.iter().map(Pair::to_json).collect::<Vec<_>>();
        config.insert("data".to_string(), data.into());

        Ok(config.into())
    }
}
