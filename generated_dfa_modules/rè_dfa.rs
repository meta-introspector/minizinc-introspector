use regex::Regex;

pub fn matches_rè(text: &str) -> bool {
    let pattern = r"^(règle)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
