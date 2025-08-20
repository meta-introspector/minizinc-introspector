use regex::Regex;

pub fn matches_s9(text: &str) -> bool {
    let pattern = r"^(s9)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
