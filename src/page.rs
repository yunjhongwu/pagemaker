use crate::component::ChartObject;
use crate::utils::{minimize, DEFAULT_CSS_PATH};
use crate::{utils, TextObject};
use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Page {
    title: Option<String>,
    content: String,
    css: String,
    include_charts: bool,
}

impl Default for Page {
    fn default() -> Self {
        Self::new(PathBuf::from(DEFAULT_CSS_PATH)).unwrap()
    }
}
impl Page {
    pub fn new(css_path: PathBuf) -> Option<Self> {
        let css = minimize(fs::read_to_string(css_path).ok()?);
        Some(Self {
            title: None,
            content: String::new(),
            css,
            include_charts: false,
        })
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());

        self
    }

    pub fn append_text(mut self, content: impl TextObject) -> Self {
        self.content.push_str(content.to_html().as_str());

        self
    }

    pub fn append_chart(mut self, content: impl ChartObject) -> Self {
        self.include_charts = true;
        self.content.push_str(content.to_html().as_str());

        self
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<html>");
        html.push_str(format!("<head><style>{}</style>", self.css).as_str());

        if let Some(title) = &self.title {
            html.push_str(format!("<title>{}</title>", title).as_str());
        }
        html.push_str("</head>");
        html.push_str("<body>");
        if self.include_charts {
            html.push_str(
                format!("<script src=\"{}\"></script>", utils::DEFAULT_CHART_JS_CDN).as_str(),
            );
        }
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
