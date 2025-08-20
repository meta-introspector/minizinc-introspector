use regex::Regex;

pub fn matches_mé(text: &str) -> bool {
    let pattern = r"^(még)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
