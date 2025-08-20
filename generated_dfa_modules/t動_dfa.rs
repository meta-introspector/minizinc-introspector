use regex::Regex;

pub fn matches_t動(text: &str) -> bool {
    let pattern = r"^(t動詞)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
