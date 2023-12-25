use crate::component::object::Object;
use crate::component::{Config, Field, TextObject};
use anyhow::Result;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone)]
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

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
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
    fn to_html(&self) -> Result<String> {
        let fields = self
            .fields
            .iter()
            .filter_map(|field| field.to_html().ok())
            .collect::<Vec<_>>()
            .join("");
        Ok(format!(
            "<div class=\"row\" {}>{}</div>",
            self.config.get_style(),
            fields
        ))
    }
}

impl TextObject for Row {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
