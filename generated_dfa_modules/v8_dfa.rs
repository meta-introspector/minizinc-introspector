use regex::Regex;

pub fn matches_v8(text: &str) -> bool {
    let pattern = r"^(v8_0|v8wawrgebbjb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
