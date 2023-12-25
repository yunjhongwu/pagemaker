use minify_html::minify;
use rand::distributions::{Alphanumeric, DistString};

pub(crate) const DEFAULT_CSS_PATH: &str = "resources/styles.css";
pub(crate) const DEFAULT_CHART_JS_CDN: [&str; 3] = [
    "https://cdn.jsdelivr.net/npm/chart.js@^4/dist/chart.umd.min.js",
    "https://cdn.jsdelivr.net/npm/moment@^2/moment.min.js",
    "https://cdn.jsdelivr.net/npm/chartjs-adapter-moment@^1/dist/chartjs-adapter-moment.min.js",
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
    if let Some(number_part) = text.strip_suffix('%') {
        Some(number_part.parse::<f64>().ok()? / 100.0)
    } else {
        text.parse::<f64>().ok()
    }
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
