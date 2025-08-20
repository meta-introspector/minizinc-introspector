use regex::Regex;

pub fn matches_iq(text: &str) -> bool {
    let pattern = r"^(iq|iqn|iqu|ique|iquery|iques|iquest)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
