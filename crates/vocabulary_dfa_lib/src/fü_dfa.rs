use regex::Regex;

pub fn matches_fü(text: &str) -> bool {
    let pattern = r"^(für)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
