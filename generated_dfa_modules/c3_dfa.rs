use regex::Regex;

pub fn matches_c3(text: &str) -> bool {
    let pattern = r"^(c3txtvaghxq5xo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
