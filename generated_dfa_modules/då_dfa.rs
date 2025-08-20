use regex::Regex;

pub fn matches_då(text: &str) -> bool {
    let pattern = r"^(då)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
