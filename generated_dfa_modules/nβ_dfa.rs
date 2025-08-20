use regex::Regex;

pub fn matches_nβ(text: &str) -> bool {
    let pattern = r"^(nβ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
