use regex::Regex;

pub fn matches_aβ(text: &str) -> bool {
    let pattern = r"^(aβc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
