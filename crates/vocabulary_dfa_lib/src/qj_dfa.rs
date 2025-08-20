use regex::Regex;

pub fn matches_qj(text: &str) -> bool {
    let pattern = r"^(qj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
