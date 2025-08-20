use regex::Regex;

pub fn matches_cₙ(text: &str) -> bool {
    let pattern = r"^(cₙ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
