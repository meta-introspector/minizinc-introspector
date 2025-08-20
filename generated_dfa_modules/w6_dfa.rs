use regex::Regex;

pub fn matches_w6(text: &str) -> bool {
    let pattern = r"^(w6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
