use regex::Regex;

pub fn matches_r次(text: &str) -> bool {
    let pattern = r"^(r次の行)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
