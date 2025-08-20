use regex::Regex;

pub fn matches_nã(text: &str) -> bool {
    let pattern = r"^(não)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
