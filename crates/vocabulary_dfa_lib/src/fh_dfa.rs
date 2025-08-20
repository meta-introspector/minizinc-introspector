use regex::Regex;

pub fn matches_fh(text: &str) -> bool {
    let pattern = r"^(fh|fhgd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
