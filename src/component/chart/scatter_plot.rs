use crate::component::chart::data::Data;
use crate::component::{ChartObject, Object};
use crate::utils::get_tag;

#[derive(Debug)]
pub struct ScatterPlot {
    id: String,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    datasets: Vec<Data>,
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

    pub fn add_data(mut self, data: Data) -> Self {
        self.datasets.push(data);

        self
    }

    fn datasets_to_html(&self) -> String {
        self.datasets
            .iter()
            .map(|data| data.to_html())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Object for ScatterPlot {
    fn to_html(&self) -> String {
        let mut html = format!("<div><canvas id=\"{}\"></canvas></div>", self.id);
        html.push_str("<script>");
        html.push_str(format!("const ctx = document.getElementById('{}');", self.id).as_str());

        html.push_str(
            format!(
                "const config = {{
          type: 'scatter',
          data: {{datasets:[{}]}},
          options: {{
            scales: {{
              x: {{
                type: 'linear',
                position: 'bottom'
              }}
            }}
          }}
        }};",
                &self.datasets_to_html()
            )
            .as_str(),
        );

        html.push_str("const scatter_plot = new Chart(ctx, config);");
        html.push_str("</script>");

        html
    }
}

impl ChartObject for ScatterPlot {}
