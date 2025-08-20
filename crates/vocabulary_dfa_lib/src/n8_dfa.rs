use regex::Regex;

pub fn matches_n8(text: &str) -> bool {
    let pattern = r"^(n80|n888|n8888)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
