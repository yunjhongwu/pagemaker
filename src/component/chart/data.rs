use anyhow::Result;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Pair<XType: Display> {
    x: XType,
    y: f64,
}

impl<XType: Display> Pair<XType> {
    pub fn new(x: XType, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_html(&self) -> String {
        format!("{{x:{},y:{}}}", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Dataset<XType: Display> {
    label: Option<String>,
    data: Vec<Pair<XType>>,
}

impl<XType: Display> Dataset<XType> {
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

    pub fn to_html(&self) -> Result<String> {
        let mut html = String::from("{");
        if let Some(label) = &self.label {
            html.push_str(format!("label:\"{}\",", label).as_str());
        }
        html.push_str("data:[");
        html.push_str(
            &self
                .data
                .iter()
                .map(Pair::to_html)
                .collect::<Vec<_>>()
                .join(","),
        );
        html.push_str("]}");

        Ok(html)
    }
}
