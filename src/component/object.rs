use crate::component::Config;
use crate::Style;
use anyhow::Result;
use serde_json::Value;

pub trait Object: Sized {
    fn to_html(&self) -> Result<String>;
}

pub trait ChartObject: Object {}

pub trait TextObject: Object {
    fn set_style(mut self, style: Style, value: impl Into<Value>) -> Self {
        self.get_mut_config().set_style(style, value.into());

        self
    }

    fn get_config(&self) -> &Config;

    fn get_mut_config(&mut self) -> &mut Config;
}
