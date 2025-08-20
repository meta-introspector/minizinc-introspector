use regex::Regex;

pub fn matches_tú(text: &str) -> bool {
    let pattern = r"^(tú)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
