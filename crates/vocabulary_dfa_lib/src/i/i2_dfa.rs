use regex::Regex;

pub fn matches_i2(text: &str) -> bool {
    let pattern = r"^(i2a|i2b|i2t)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
