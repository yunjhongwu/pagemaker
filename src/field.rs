use crate::config::Config;
use crate::object::Object;

#[derive(Debug)]
pub struct Field {
    name: String,
    config: Config,
}

impl Field {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            config: Config::default(),
        }
    }
}

impl Object for Field {
    fn to_html(&self) -> String {
        let config = self.get_config();
        // create html with config
        let mut html = String::from(format!(
            "<div class=\"field\" style=\"{}\">",
            config.get_style()
        ));
        html.push_str(self.name.as_str());
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
