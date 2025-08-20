use regex::Regex;

pub fn matches_qf(text: &str) -> bool {
    let pattern = r"^(qfr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
