use regex::Regex;

pub fn matches_qv(text: &str) -> bool {
    let pattern = r"^(qvq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
