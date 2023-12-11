use crate::config::Config;
use crate::Object;

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

    pub fn add_column(mut self, field: impl Object) -> Self {
        self.columns.push(field.to_html());

        self
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }
}

impl Object for Columns {
    fn to_html(&self) -> String {
        let mut html = format!("<div class=\"columns\" {}>", self.get_config().get_style());
        let width = format!("width:{}%;", 100.0 / self.columns.len() as f32);

        for column in &self.columns {
            html.push_str(format!("<div class=\"column\" style={}>", width).as_str());
            html.push_str(column.as_str());
            html.push_str("</div>");
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
