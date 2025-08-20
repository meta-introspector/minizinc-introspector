use regex::Regex;

pub fn matches_sí(text: &str) -> bool {
    let pattern = r"^(sí)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
