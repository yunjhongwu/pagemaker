use crate::component::Config;

pub trait Object: Sized {
    fn to_html(&self) -> String;

    fn set_text_color(&mut self, color_string: impl Into<String>) -> &mut Self {
        self.get_mut_config().set_text_color(color_string);

        self
    }

    fn set_background_color(&mut self, color_string: impl Into<String>) -> &mut Self {
        self.get_mut_config().set_background_color(color_string);

        self
    }

    fn set_font_size(&mut self, size: u32) -> &mut Self {
        self.get_mut_config().set_font_size(size);

        self
    }

    fn get_config(&self) -> &Config;

    fn get_mut_config(&mut self) -> &mut Config;
}
