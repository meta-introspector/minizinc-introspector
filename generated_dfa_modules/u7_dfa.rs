use regex::Regex;

pub fn matches_u7(text: &str) -> bool {
    let pattern = r"^(u7|u7ue)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
