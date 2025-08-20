use regex::Regex;

pub fn matches_f4(text: &str) -> bool {
    let pattern = r"^(f4_len|f4x)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
