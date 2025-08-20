use regex::Regex;

pub fn matches_w1(text: &str) -> bool {
    let pattern = r"^(w10|w14b|w1_8b|w1_b|w1_span|w1_str|w1b50b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
