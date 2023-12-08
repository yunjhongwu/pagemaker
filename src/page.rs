use crate::Object;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const CSS_PATH: &str = "resources/styles.css";

fn minimize(string: String) -> String {
    let re = Regex::new(r"\s+").unwrap();

    re.replace_all(string.as_str(), " ")
        .to_string()
        .replace("\n", "")
}

#[derive(Debug, Default)]
pub struct Page {
    title: Option<String>,
    content: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            title: None,
            content: String::new(),
        }
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());

        self
    }

    pub fn append(mut self, content: impl Object) -> Self {
        self.content.push_str(content.to_html().as_str());

        self
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<html>");
        html.push_str("<head><style>");

        if let Ok(css) = fs::read_to_string(CSS_PATH) {
            html.push_str(minimize(css).as_str());
        }
        html.push_str("</style>");
        if let Some(title) = &self.title {
            html.push_str(format!("<title>{}</title>", title).as_str());
        }
        html.push_str("</head>");
        html.push_str("<body>");
        html.push_str("<div class=\"page\">");
        html.push_str(self.content.as_str());
        html.push_str("</div>");
        html.push_str("</body>");
        html.push_str("</html>");

        html
    }

    pub fn save_to_html(&self, filepath: PathBuf) -> Result<()> {
        let mut output = File::create(filepath)?;

        Ok(write!(output, "{}", self.to_html())?)
    }
}
