use regex::Regex;

pub fn matches_w8(text: &str) -> bool {
    let pattern = r"^(w8)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
