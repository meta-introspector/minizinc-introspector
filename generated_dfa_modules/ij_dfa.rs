use regex::Regex;

pub fn matches_ij(text: &str) -> bool {
    let pattern = r"^(ijg|ijklm|ijl|ijli|ijlig)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
