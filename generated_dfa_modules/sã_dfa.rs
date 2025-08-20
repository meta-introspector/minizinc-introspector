use regex::Regex;

pub fn matches_sã(text: &str) -> bool {
    let pattern = r"^(são)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
