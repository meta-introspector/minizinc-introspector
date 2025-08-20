use regex::Regex;

pub fn matches_s6(text: &str) -> bool {
    let pattern = r"^(s6_addr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
