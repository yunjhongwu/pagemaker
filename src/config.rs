use crate::color_utils::validate_color;

#[derive(Debug, Default)]
pub struct Config {
    text_color: Option<String>,
    background_color: Option<String>,
    font_size: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            text_color: None,
            background_color: None,
            font_size: None,
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

    pub fn set_font_size(&mut self, size: u32) -> &Self {
        self.font_size = Some(size.to_string());

        self
    }

    pub fn get_style(&self) -> String {
        let mut styles = String::from("");

        if let Some(color) = &self.text_color {
            styles.push_str(format!("color:{};", color).as_str());
        }

        if let Some(style) = &self.background_color {
            styles.push_str(format!("background-color:{};", style).as_str());
        }

        if let Some(size) = &self.font_size {
            styles.push_str(format!("font-size:{}px;", size).as_str());
        }

        if styles.is_empty() {
            String::from("")
        } else {
            format!("style=\"{}\"", styles)
        }
    }
}
