use minify_html::minify;
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;

pub(crate) const DEFAULT_CSS_PATH: &str = "resources/styles.css";
pub(crate) const DEFAULT_CHART_JS_CDN: [&str; 3] = [
    "https://cdn.jsdelivr.net/npm/chart.js",
    "https://cdn.jsdelivr.net/npm/moment@^2",
    "https://cdn.jsdelivr.net/npm/chartjs-adapter-moment@^1",
];

pub(crate) fn minimize(string: String) -> Vec<u8> {
    let cfg = minify_html::Cfg {
        minify_css: true,
        minify_js: true,
        ..Default::default()
    };
    minify(string.as_bytes(), &cfg)
}

pub(crate) fn string_to_value(text: &str) -> Option<f64> {
    let re = Regex::new(r"^\s*(\d+(\.\d+)?)\s*(%?)\s*$").ok()?;
    let captures = re.captures(text).unwrap();
    let value = captures.get(1).unwrap().as_str().parse::<f64>().ok()?;
    let unit = captures.get(3).unwrap().as_str();
    let output = if unit == "%" { value / 100.0 } else { value };

    Some(output)
}

pub(crate) fn get_tag(prefix: &str) -> String {
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    format!("{}-{}", prefix, id)
}

#[cfg(test)]
mod tests {
    use crate::utils::get_tag;

    #[test]
    fn test_get_tag() {
        let tag1 = get_tag("test");
        let tag2 = get_tag("test");
        assert_eq!(tag1.len(), 21);
        assert_ne!(tag1, tag2);
    }
}
