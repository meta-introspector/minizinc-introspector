use regex::Regex;

pub fn matches_mö(text: &str) -> bool {
    let pattern = r"^(möh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
