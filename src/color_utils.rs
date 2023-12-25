use crate::utils::string_to_value;
use regex::Regex;

pub mod color {
    // python seaborn color palette
    pub const BLUE: &str = "#4c72b0";
    pub const GREEN: &str = "#55a868";
    pub const RED: &str = "#c44e52";
    pub const PURPLE: &str = "#8172b2";
    pub const YELLOW: &str = "#ccb974";
    pub const CYAN: &str = "#64b5cd";
    pub const MAGENTA: &str = "#4c72b0";
    pub const ORANGE: &str = "#ff9e4a";
}

#[allow(unused)]
pub struct ColorKnot(f64, String);
pub struct ColorMap {
    knots: Vec<(f64, String)>,
}

impl ColorMap {
    pub fn new(knots: Vec<(f64, String)>) -> Option<Self> {
        let mut knots = knots;
        if knots
            .iter()
            .any(|(value, color)| !value.is_finite() || validate_color(color.as_str()).is_none())
        {
            None
        } else {
            knots.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            Some(Self { knots })
        }
    }

    pub fn get_color(&self, content: impl Into<String>) -> Option<String> {
        let value = string_to_value(content.into().as_str())?;

        if value < self.knots[0].0 {
            return Some(self.knots[0].1.clone().to_lowercase());
        }
        if value >= self.knots[self.knots.len() - 1].0 {
            return Some(self.knots[self.knots.len() - 1].1.clone().to_lowercase());
        }
        let mut color = String::from("");
        let mut last_knot = &self.knots[0];
        for knot in self.knots.iter() {
            if value < knot.0 {
                let ratio = (value - last_knot.0) / (knot.0 - last_knot.0);
                color = self.interpolate_color(last_knot.1.as_str(), knot.1.as_str(), ratio);
                break;
            }
            last_knot = &knot;
        }

        Some(color)
    }

    fn interpolate_color(&self, color1: &str, color2: &str, ratio: f64) -> String {
        let mut color = String::from("#");
        let num_channels = if color1.len() == 7 { 3 } else { 4 };
        for i in 0..num_channels {
            let channel_1 = u8::from_str_radix(&color1[2 * i + 1..2 * i + 3], 16).unwrap();
            let channel_2 = u8::from_str_radix(&color2[2 * i + 1..2 * i + 3], 16).unwrap();
            let channel = (channel_1 as f64 * (1.0 - ratio) + channel_2 as f64 * ratio) as u8;

            color.push_str(format!("{:02x}", channel).as_str());
        }

        color
    }
}

pub(crate) fn validate_color(color: impl Into<String>) -> Option<String> {
    let re = Regex::new(r"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
    let color = color.into();
    match re.is_match(color.as_str()) {
        true => Some(color.to_lowercase()),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::color_utils::{validate_color, ColorMap};

    #[test]
    fn test_validate_color() {
        assert!(validate_color("#4C72B0").is_some());
        assert!(validate_color("#4c72B012").is_some());
        assert!(validate_color("x4C72B0").is_none());
        assert!(validate_color("#4h72B012").is_none());
    }

    #[test]
    fn test_color_interpolator() {
        let knots = vec![
            (0.0, "#4C72B0".to_string()),
            (0.5, "#55A868".to_string()),
            (1.0, "#C44E52".to_string()),
        ];
        let interpolator = ColorMap::new(knots).unwrap();
        assert_eq!(interpolator.get_color("0.0").unwrap(), "#4c72b0");
        assert_eq!(interpolator.get_color("0.25").unwrap(), "#508d8c");
        assert_eq!(interpolator.get_color("0.5").unwrap(), "#55a868");
        assert_eq!(interpolator.get_color("0.75").unwrap(), "#8c7b5d");
        assert_eq!(interpolator.get_color("1.0").unwrap(), "#c44e52");
        assert_eq!(interpolator.get_color("25%").unwrap(), "#508d8c");
        assert!(interpolator.get_color("NAN").is_none());
    }
}
