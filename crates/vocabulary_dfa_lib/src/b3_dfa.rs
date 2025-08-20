use regex::Regex;

pub fn matches_b3(text: &str) -> bool {
    let pattern = r"^(b3_0|b3_1|b3g4)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
