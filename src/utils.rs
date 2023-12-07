use regex::Regex;

pub(crate) fn validate_color(color: &str) -> Option<&str> {
    let re = Regex::new(r"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
    if re.is_match(color) {
        Some(color)
    } else {
        None
    }
}

pub mod colors {
    pub const BLUE: &str = "#4C72B0";
    pub const ORANGE: &str = "#DD8452";
    pub const GREEN: &str = "#55A868";
    pub const RED: &str = "#C44E52";
    pub const PURPLE: &str = "#8172B2";
    pub const BROWN: &str = "#937860";
    pub const PINK: &str = "#DA8BC3";
    pub const GRAY: &str = "#8C8C8C";
    pub const YELLOW: &str = "#CCB974";
    pub const CYAN: &str = "#64B5CD";
}
