use regex::Regex;

pub fn matches_nț(text: &str) -> bool {
    let pattern = r"^(nțc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
