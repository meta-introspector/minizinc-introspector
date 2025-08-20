use regex::Regex;

pub fn matches_hé(text: &str) -> bool {
    let pattern = r"^(hé_hé)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
