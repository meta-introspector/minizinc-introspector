use regex::Regex;

pub fn matches_r6(text: &str) -> bool {
    let pattern = r"^(r6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
