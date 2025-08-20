use regex::Regex;

pub fn matches_v5(text: &str) -> bool {
    let pattern = r"^(v5_0|v5te)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
