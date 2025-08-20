use regex::Regex;

pub fn matches_r8(text: &str) -> bool {
    let pattern = r"^(r8g8b8|r8g8b8a8|r8rw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
