use crate::chart::data::Dataset;
use crate::chart::plot_type::{PlotType, XType};
use crate::component::{ChartObject, Object};
use crate::utils::get_tag;
use anyhow::Result;
use std::marker::PhantomData;

use serde_json::{json, Map};

pub mod plot_type {
    use serde::Serialize;
    use serde_json::{json, Value};
    use std::fmt::{Debug, Display};
    use std::marker::PhantomData;

    pub trait PlotType {
        fn get_str() -> String {
            unimplemented!("PlotType::get_str() is not implemented");
        }
    }

    pub struct Scatter;
    impl PlotType for Scatter {
        fn get_str() -> String {
            "scatter".to_string()
        }
    }

    pub struct Line;
    impl PlotType for Line {
        fn get_str() -> String {
            "line".to_string()
        }
    }

    pub trait XType {
        type ValueType: Debug + Display + Serialize;
        fn get_str() -> String {
            unimplemented!("PlotType::get_str() is not implemented");
        }

        fn get_unit() -> Option<Value> {
            None
        }
    }

    pub struct Linear<T: Display + Serialize> {
        _phantom: PhantomData<T>,
    }
    impl<T: Debug + Display + Serialize> XType for Linear<T> {
        type ValueType = T;
        fn get_str() -> String {
            "linear".to_string()
        }
    }

    pub struct Time<T: Display + Serialize> {
        _phantom: PhantomData<T>,
    }
    impl<T: Debug + Display + Serialize> XType for Time<T> {
        type ValueType = T;
        fn get_str() -> String {
            "time".to_string()
        }

        fn get_unit() -> Option<Value> {
            Some(json!({"parser": "x", "unit": "day",
            "tooltipFormat": "YYYY-MM-DD",
                    "displayFormats": {
                        "day": "YYYY-MM-DD"
                    }}))
        }
    }
}

#[derive(Debug)]
pub struct XYPlot<T: PlotType, X: XType> {
    id: String,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    datasets: Vec<Dataset<<X as XType>::ValueType>>,
    plot_type: PhantomData<T>,
}

impl<T: PlotType, X: XType> Default for XYPlot<T, X> {
    fn default() -> Self {
        Self {
            id: get_tag(T::get_str().as_str()),
            title: None,
            x_label: None,
            y_label: None,
            datasets: Vec::new(),
            plot_type: PhantomData,
        }
    }
}

impl<T: PlotType, X: XType> XYPlot<T, X> {
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

    pub fn add_data(mut self, data: Dataset<<X as XType>::ValueType>) -> Self {
        self.datasets.push(data);

        self
    }
}

impl<T: PlotType, X: XType> Object for XYPlot<T, X> {
    fn to_html(&self) -> Result<String> {
        let context = format!("document.getElementById('{}')", self.id);
        let mut config = Map::new();
        config.insert("type".to_string(), T::get_str().into());

        let mut data = Map::new();

        let datasets = self
            .datasets
            .iter()
            .filter_map(|dataset| dataset.to_json().ok())
            .collect::<Vec<_>>();
        data.insert("datasets".to_string(), datasets.into());
        config.insert("data".to_string(), data.into());

        let mut options = Map::new();
        let mut x = Map::new();
        x.insert("type".to_string(), X::get_str().into());
        if let Some(unit) = X::get_unit() {
            x.insert("time".to_string(), unit);
        }
        x.insert("position".to_string(), "bottom".into());
        if let Some(x_label) = &self.x_label {
            x.insert(
                "title".to_string(),
                json!({"display": true, "text": x_label}),
            );
        }

        let mut y = Map::new();
        if let Some(y_label) = &self.y_label {
            y.insert(
                "title".to_string(),
                json!({"display": true, "text": y_label}),
            );
        }

        options.insert("scales".to_string(), json!({"x": x, "y": y}));
        if let Some(title) = &self.title {
            options.insert(
                "plugins".to_string(),
                json!({"title": {"display": true, "text": title}}),
            );
        }
        config.insert("options".to_string(), options.into());

        let html = format!(
            "<div><canvas id=\"{}\"></canvas></div><script>document.addEventListener('DOMContentLoaded', function () {{ new Chart({}, {}); }});</script>",
            self.id,
            context,
            json!(config)
        );
        Ok(html)
    }
}

impl<T: PlotType, X: XType> ChartObject for XYPlot<T, X> {}
