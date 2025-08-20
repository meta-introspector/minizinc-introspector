use regex::Regex;

pub fn matches_lj(text: &str) -> bool {
    let pattern = r"^(lj|ljc|ljcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
