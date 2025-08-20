use regex::Regex;

pub fn matches_p9(text: &str) -> bool {
    let pattern = r"^(p99|p999)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
