use regex::Regex;

pub fn matches_fé(text: &str) -> bool {
    let pattern = r"^(février)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
