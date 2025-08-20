use regex::Regex;

pub fn matches_e0(text: &str) -> bool {
    let pattern = r"^(e0euymm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
