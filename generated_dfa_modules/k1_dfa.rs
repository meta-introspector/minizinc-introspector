use regex::Regex;

pub fn matches_k1(text: &str) -> bool {
    let pattern = r"^(k12)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
