use regex::Regex;

pub fn matches_fä(text: &str) -> bool {
    let pattern = r"^(fängt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
