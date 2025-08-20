use regex::Regex;

pub fn matches_bâ(text: &str) -> bool {
    let pattern = r"^(bâton)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
