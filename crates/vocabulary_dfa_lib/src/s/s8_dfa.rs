use regex::Regex;

pub fn matches_s8(text: &str) -> bool {
    let pattern = r"^(s8b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
