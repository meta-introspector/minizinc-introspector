use regex::Regex;

pub fn matches_kz(text: &str) -> bool {
    let pattern = r"^(kz|kzw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
