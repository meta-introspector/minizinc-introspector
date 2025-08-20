use regex::Regex;

pub fn matches_oy(text: &str) -> bool {
    let pattern = r"^(oy_vey|oyster|oystercatcher)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
