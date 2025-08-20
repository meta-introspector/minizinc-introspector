use regex::Regex;

pub fn matches_t6(text: &str) -> bool {
    let pattern = r"^(t60|t6a|t6b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
