use regex::Regex;

pub fn matches_rᵢ(text: &str) -> bool {
    let pattern = r"^(rᵢ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
