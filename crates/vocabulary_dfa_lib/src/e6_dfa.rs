use regex::Regex;

pub fn matches_e6(text: &str) -> bool {
    let pattern = r"^(e600_r384)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
