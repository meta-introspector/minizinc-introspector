use regex::Regex;

pub fn matches_så(text: &str) -> bool {
    let pattern = r"^(så|sådan|sådana|sådant|sånn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
