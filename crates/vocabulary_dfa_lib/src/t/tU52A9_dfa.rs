use regex::Regex;

pub fn matches_t助(text: &str) -> bool {
    let pattern = r"^(t助詞)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
