use crate::component::object::Object;
use crate::component::{Config, TextObject};
use crate::Style;
use anyhow::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Field {
    content: String,
    config: Config,
}

impl Field {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            config: Config::default(),
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn apply_style_map<F: Fn(&String) -> Option<Value>>(
        &mut self,
        style: Style,
        style_map: F,
    ) -> &mut Self {
        if let Some(value) = style_map(self.get_content()) {
            self.get_mut_config().set_style(style, value);
        }

        self
    }
}

impl Object for Field {
    fn to_html(&self) -> Result<String> {
        Ok(format!(
            "<div class=\"field\" {}>{}</div>",
            self.get_config().get_style(),
            self.content
        ))
    }
}

impl TextObject for Field {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
