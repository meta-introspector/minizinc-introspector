use regex::Regex;

pub fn matches_k7(text: &str) -> bool {
    let pattern = r"^(k7mdeng)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
