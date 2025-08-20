use regex::Regex;

pub fn matches_t8(text: &str) -> bool {
    let pattern = r"^(t80)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
