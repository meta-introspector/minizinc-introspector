use regex::Regex;

pub fn matches_nα(text: &str) -> bool {
    let pattern = r"^(nαβ|nαβγ|nαβγδε)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
