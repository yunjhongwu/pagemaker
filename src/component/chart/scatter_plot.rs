use crate::chart::data::Dataset;
use crate::component::{ChartObject, Object};
use crate::utils::get_tag;
use anyhow::Result;

#[derive(Debug)]
pub struct ScatterPlot {
    id: String,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    datasets: Vec<Dataset>,
}

impl Default for ScatterPlot {
    fn default() -> Self {
        Self {
            id: get_tag("scatter-plot"),
            title: None,
            x_label: None,
            y_label: None,
            datasets: Vec::new(),
        }
    }
}

impl ScatterPlot {
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());

        self
    }

    pub fn set_x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());

        self
    }

    pub fn set_y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());

        self
    }

    pub fn add_data(mut self, data: Dataset) -> Self {
        self.datasets.push(data);

        self
    }

    fn datasets_to_html(&self) -> Result<String> {
        let mut datasets_html = Vec::<String>::new();
        for dataset in &self.datasets {
            datasets_html.push(dataset.to_html()?);
        }

        Ok(datasets_html.join(","))
    }
}

impl Object for ScatterPlot {
    fn to_html(&self) -> Result<String> {
        let mut html = format!("<div><canvas id=\"{}\"></canvas></div>", self.id);
        html.push_str("<script>");
        let context = format!("document.getElementById('{}')", self.id);

        let mut config = String::from("{");
        config.push_str("type:'scatter',");
        config.push_str(format!("data:{{datasets:[{}]}},", self.datasets_to_html()?).as_str());
        config.push_str("options:{scales:{x:{type:'linear',position:'bottom'}}}}");

        html.push_str(format!("const scatter_plot = new Chart({}, {});", context, config).as_str());
        html.push_str("</script>");

        Ok(html)
    }
}

impl ChartObject for ScatterPlot {}
