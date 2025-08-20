use regex::Regex;

pub fn matches_på(text: &str) -> bool {
    let pattern = r"^(på)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
