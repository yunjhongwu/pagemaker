use crate::config::Config;
use crate::object::Object;
use crate::Field;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Row {
    fields: Vec<Field>,
    config: Config,
}

impl Row {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            config: Default::default(),
        }
    }

    pub fn from_vec(fields: Vec<Field>) -> Self {
        Self {
            fields,
            config: Default::default(),
        }
    }

    pub fn set(mut self, field: Field, index: usize) -> Self {
        self.fields[index] = field;

        self
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.fields.push(field);

        self
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn length(&self) -> usize {
        self.fields.len()
    }
}

impl Index<usize> for Row {
    type Output = Field;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}

impl Object for Row {
    fn to_html(&self) -> String {
        let mut html = format!("<div class=\"row\" style=\"{}\">", self.config.get_style());
        for field in self.fields.iter() {
            html.push_str(field.to_html().as_str());
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
