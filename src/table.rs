use crate::config::Config;
use crate::object::Object;
use crate::Row;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
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
        if row.length() != self.rows[0].length() {
            panic!("Row length does not match header length.");
        }
        self.rows.push(row);

        self
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

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
