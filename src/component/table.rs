use crate::component::object::Object;
use crate::component::style::Style;
use crate::component::{Config, Row, TextObject};
use crate::utils::string_to_value;
use crate::ColorMap;
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

    pub fn apply_to_column(&mut self, col_idx: usize, colormap: &ColorMap, style: Style) {
        for row in self.rows.iter_mut() {
            if let Some(value) = string_to_value(row[col_idx].get_content()) {
                let color = colormap.get_color(value);
                match style {
                    Style::Text => row[col_idx].set_text_color(color),
                    Style::Background => row[col_idx].set_background_color(color),
                };
            }
        }
    }

    pub fn apply_to_row(&mut self, row_idx: usize, colormap: &ColorMap, style: Style) {
        let row = &mut self.rows[row_idx];
        for i in 0..row.len() {
            let field = &mut row[i];
            if let Some(value) = string_to_value(field.get_content()) {
                let color = colormap.get_color(value);
                match style {
                    Style::Text => field.set_text_color(color),
                    Style::Background => field.set_background_color(color),
                };
            };
        }
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
    fn to_html(&self) -> String {
        let mut html = String::new();
        if let Some(title) = &self.title {
            html.push_str(format!("<h3>{}</h3>", title).as_str());
        }

        html.push_str("<div class=\"table\">");
        for row in self.rows.iter() {
            html.push_str(row.to_html().as_str());
        }
        html.push_str("</div>");

        html
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
