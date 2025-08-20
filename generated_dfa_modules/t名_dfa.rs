use regex::Regex;

pub fn matches_t名(text: &str) -> bool {
    let pattern = r"^(t名詞)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
