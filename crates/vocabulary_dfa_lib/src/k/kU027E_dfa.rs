use regex::Regex;

pub fn matches_kɾ(text: &str) -> bool {
    let pattern = r"^(kɾiʃ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
