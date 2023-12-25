use crate::component::object::Object;
use crate::component::{Config, TextObject};
use anyhow::Result;

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

    pub fn get_content(&self) -> &str {
        self.content.as_str()
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
