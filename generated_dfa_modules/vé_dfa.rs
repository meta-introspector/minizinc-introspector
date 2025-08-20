use regex::Regex;

pub fn matches_vé(text: &str) -> bool {
    let pattern = r"^(vénus)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
