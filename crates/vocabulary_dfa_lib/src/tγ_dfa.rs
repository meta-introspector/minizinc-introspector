use regex::Regex;

pub fn matches_tγ(text: &str) -> bool {
    let pattern = r"^(tγ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
