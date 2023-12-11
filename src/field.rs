use crate::config::Config;
use crate::object::Object;

#[derive(Debug)]
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
}

impl Object for Field {
    fn to_html(&self) -> String {
        let config = self.get_config();
        let mut html = format!("<div class=\"field\" {}>", config.get_style());
        html.push_str(self.content.as_str());
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
