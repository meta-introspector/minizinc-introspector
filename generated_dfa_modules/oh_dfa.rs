use regex::Regex;

pub fn matches_oh(text: &str) -> bool {
    let pattern = r"^(oh_row|ohb|ohba|ohbar|ohm|ohne|ohos)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
