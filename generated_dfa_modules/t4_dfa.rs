use regex::Regex;

pub fn matches_t4(text: &str) -> bool {
    let pattern = r"^(t40|t4a|t4b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
