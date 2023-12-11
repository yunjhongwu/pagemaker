use regex::Regex;

pub(crate) const DEFAULT_CSS_PATH: &str = "resources/styles.css";

pub(crate) fn minimize(string: String) -> String {
    let re = Regex::new(r"\s+").unwrap();

    re.replace_all(string.as_str(), " ")
        .to_string()
        .replace("\n", "")
}
