use regex::Regex;

pub fn matches_pj(text: &str) -> bool {
    let pattern = r"^(pj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
