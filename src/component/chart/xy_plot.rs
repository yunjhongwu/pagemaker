use crate::chart::data::Dataset;
use crate::chart::plot_type::PlotType;
use crate::component::{ChartObject, Object};
use crate::utils::get_tag;
use anyhow::Result;
use std::marker::PhantomData;

pub mod plot_type {
    pub trait PlotType<'a> {
        fn get_str() -> &'a str {
            panic!("PlotType::get_str() is not implemented");
        }
    }

    pub struct Scatter;
    impl<'a> PlotType<'a> for Scatter {
        fn get_str() -> &'a str {
            "scatter"
        }
    }

    pub struct Line;
    impl<'a> PlotType<'a> for Line {
        fn get_str() -> &'a str {
            "line"
        }
    }
}

#[derive(Debug)]
pub struct XYPlot<T: for<'a> PlotType<'a>> {
    id: String,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    datasets: Vec<Dataset<f64>>,
    plot_type: PhantomData<T>,
}

impl<T: for<'a> PlotType<'a>> Default for XYPlot<T> {
    fn default() -> Self {
        Self {
            id: get_tag("scatter-plot"),
            title: None,
            x_label: None,
            y_label: None,
            datasets: Vec::new(),
            plot_type: PhantomData,
        }
    }
}

impl<T: for<'a> PlotType<'a>> XYPlot<T> {
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

    pub fn add_data(mut self, data: Dataset<f64>) -> Self {
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

impl<T: for<'a> PlotType<'a>> Object for XYPlot<T> {
    fn to_html(&self) -> Result<String> {
        let mut html = format!("<div><canvas id=\"{}\"></canvas></div>", self.id);
        html.push_str("<script>");
        let context = format!("document.getElementById('{}')", self.id);

        let mut config = String::from("{");
        config.push_str(format!("type:'{}',", T::get_str()).as_str());
        config.push_str(format!("data:{{datasets:[{}]}},", self.datasets_to_html()?).as_str());
        config.push_str("options:{scales:{x:{type:'linear',position:'bottom'}}}}");

        html.push_str(format!("const scatter_plot = new Chart({}, {});", context, config).as_str());
        html.push_str("</script>");

        Ok(html)
    }
}

impl<T: for<'a> PlotType<'a>> ChartObject for XYPlot<T> {}
