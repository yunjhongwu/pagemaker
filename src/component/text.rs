use crate::component::object::Object;
use crate::component::Config;
use crate::TextObject;

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    config: Config,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            config: Config::default(),
        }
    }
}

impl Object for Text {
    fn to_html(&self) -> String {
        format!("<span {}>{}</span>", self.config.get_style(), self.text)
    }
}

impl TextObject for Text {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }
}
