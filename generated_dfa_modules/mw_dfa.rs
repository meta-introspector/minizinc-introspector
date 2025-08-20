use regex::Regex;

pub fn matches_mw(text: &str) -> bool {
    let pattern = r"^(mwa|mwfd|mwgxqwq|mwk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
