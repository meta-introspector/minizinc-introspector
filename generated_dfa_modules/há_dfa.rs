use regex::Regex;

pub fn matches_há(text: &str) -> bool {
    let pattern = r"^(há)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
