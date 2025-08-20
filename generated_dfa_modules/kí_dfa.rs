use regex::Regex;

pub fn matches_kí(text: &str) -> bool {
    let pattern = r"^(kívül)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
