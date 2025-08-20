use regex::Regex;

pub fn matches_b4(text: &str) -> bool {
    let pattern = r"^(b4_0|b4_1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
