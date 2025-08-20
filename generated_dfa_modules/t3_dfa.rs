use regex::Regex;

pub fn matches_t3(text: &str) -> bool {
    let pattern = r"^(t3333|t345|t3a|t3b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
