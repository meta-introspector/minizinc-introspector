use regex::Regex;

pub fn matches_mê(text: &str) -> bool {
    let pattern = r"^(même)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
