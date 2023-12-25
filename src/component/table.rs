use crate::component::object::Object;
use crate::component::style::Style;
use crate::component::{Config, Row, TextObject};
use anyhow::Result;
use serde_json::Value;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone)]
pub struct Table {
    title: Option<String>,
    rows: Vec<Row>,
    config: Config,
}

impl Table {
    pub fn new(header: Row) -> Self {
        Self {
            title: None,
            rows: Vec::from([header]),
            config: Config::default(),
        }
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());

        self
    }

    pub fn add_row(mut self, row: Row) -> Self {
        if row.len() != self.rows[0].len() {
            panic!("Row length does not match header length.");
        }
        self.rows.push(row);

        self
    }

    pub fn apply_to_column<F: Fn(&String) -> Option<Value>>(
        mut self,
        col_idx: usize,
        style: Style,
        style_map: &F,
    ) -> Self {
        for row in self.rows.iter_mut() {
            row[col_idx].apply_style_map(style, style_map);
        }

        self
    }

    pub fn apply_to_row<F: Fn(&String) -> Option<Value>>(
        mut self,
        row_idx: usize,
        style: Style,
        style_map: &F,
    ) -> Self {
        let row = &mut self.rows[row_idx];
        for i in 0..row.len() {
            let field = &mut row[i];
            field.apply_style_map(style, style_map);
        }

        self
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl Index<usize> for Table {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl Object for Table {
    fn to_html(&self) -> Result<String> {
        let mut html = String::new();
        if let Some(title) = &self.title {
            html.push_str(format!("<h3>{}</h3>", title).as_str());
        }

        html.push_str(
            format!(
                "<div class=\"table\">{}</div>",
                self.rows
                    .iter()
                    .filter_map(|row| row.to_html().ok())
                    .collect::<Vec<_>>()
                    .join("")
            )
            .as_str(),
        );

        Ok(html)
    }
}

impl TextObject for Table {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
