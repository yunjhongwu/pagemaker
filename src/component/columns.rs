use crate::component::object::Object;
use crate::component::Config;
use crate::TextObject;
use anyhow::Result;

#[derive(Debug, Default, Clone)]
pub struct Columns {
    columns: Vec<String>,
    config: Config,
}

impl Columns {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            config: Default::default(),
        }
    }

    pub fn add_column(mut self, field: impl TextObject) -> Result<Self> {
        self.columns.push(field.to_html()?);

        Ok(self)
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
}

impl Object for Columns {
    fn to_html(&self) -> Result<String> {
        let width = 100.0 / self.columns.len() as f32;

        let columns = self
            .columns
            .iter()
            .map(|column| {
                format!(
                    "<div class=\"column\" style=width:{}%>{}</div>",
                    width, column
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let html = format!(
            "<div class=\"columns\" {}>{}</div>",
            self.get_config().get_style(),
            columns
        );

        Ok(html)
    }
}

impl TextObject for Columns {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
