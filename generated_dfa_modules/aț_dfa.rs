use regex::Regex;

pub fn matches_aț(text: &str) -> bool {
    let pattern = r"^(ațc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
