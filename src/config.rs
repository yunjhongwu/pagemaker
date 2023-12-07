use crate::utils::validate_color;

#[derive(Debug)]
pub struct Config {
    text_color: Option<String>,
    background_color: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            text_color: None,
            background_color: None,
        }
    }

    pub fn set_text_color(&mut self, color_string: impl Into<String>) -> &Self {
        if let Some(color) = validate_color(color_string.into().as_str()) {
            self.text_color = Some(color.to_string());
        }

        self
    }

    pub fn set_background_color(&mut self, color_string: impl Into<String>) -> &Self {
        if let Some(color) = validate_color(color_string.into().as_str()) {
            self.background_color = Some(color.to_string());
        }

        self
    }

    pub fn get_style(&self) -> String {
        let mut style = String::from("");
        if let Some(color) = &self.text_color {
            style.push_str(format!("color: {}; ", color).as_str());
        }
        if let Some(color) = &self.background_color {
            style.push_str(format!("background-color: {}; ", color).as_str());
        }

        style
    }
}
