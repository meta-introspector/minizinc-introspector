use regex::Regex;

pub fn matches_tα(text: &str) -> bool {
    let pattern = r"^(tα)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
