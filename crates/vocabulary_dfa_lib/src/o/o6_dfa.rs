use regex::Regex;

pub fn matches_o6(text: &str) -> bool {
    let pattern = r"^(o6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
