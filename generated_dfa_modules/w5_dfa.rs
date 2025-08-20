use regex::Regex;

pub fn matches_w5(text: &str) -> bool {
    let pattern = r"^(w5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
