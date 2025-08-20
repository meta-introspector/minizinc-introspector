use regex::Regex;

pub fn matches_q0(text: &str) -> bool {
    let pattern = r"^(q0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
