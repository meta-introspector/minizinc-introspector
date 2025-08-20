use regex::Regex;

pub fn matches_sé(text: &str) -> bool {
    let pattern = r"^(sélection)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
