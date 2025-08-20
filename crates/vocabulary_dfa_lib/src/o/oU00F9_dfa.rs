use regex::Regex;

pub fn matches_où(text: &str) -> bool {
    let pattern = r"^(où)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
