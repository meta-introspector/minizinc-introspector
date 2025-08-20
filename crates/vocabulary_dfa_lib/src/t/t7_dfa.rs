use regex::Regex;

pub fn matches_t7(text: &str) -> bool {
    let pattern = r"^(t7a|t7b|t7dkb5ki7q)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
