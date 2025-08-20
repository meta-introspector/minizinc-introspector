use regex::Regex;

pub fn matches_n6(text: &str) -> bool {
    let pattern = r"^(n64|n666|n6666|n67)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
