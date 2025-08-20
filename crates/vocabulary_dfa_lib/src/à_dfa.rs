use regex::Regex;

pub fn matches_à(text: &str) -> bool {
    let pattern = r"^(à)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
