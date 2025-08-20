use regex::Regex;

pub fn matches_t2(text: &str) -> bool {
    let pattern = r"^(t20|t200|t2a|t2b|t2i)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
