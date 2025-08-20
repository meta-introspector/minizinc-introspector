use regex::Regex;

pub fn matches_w9(text: &str) -> bool {
    let pattern = r"^(w9)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
