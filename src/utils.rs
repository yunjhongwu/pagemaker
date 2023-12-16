use regex::Regex;

pub(crate) const DEFAULT_CSS_PATH: &str = "resources/styles.css";

pub(crate) fn minimize(string: String) -> String {
    let re = Regex::new(r"\s+").unwrap();

    re.replace_all(string.as_str(), " ")
        .to_string()
        .replace('\n', "")
}

pub(crate) fn string_to_value(text: &str) -> Option<f64> {
    let re = Regex::new(r"^\s*(\d+(\.\d+)?)\s*(%?)\s*$").ok()?;
    let captures = re.captures(text).unwrap();
    let value = captures.get(1).unwrap().as_str().parse::<f64>().ok()?;
    let unit = captures.get(3).unwrap().as_str();
    let output = if unit == "%" { value / 100.0 } else { value };

    Some(output)
}
