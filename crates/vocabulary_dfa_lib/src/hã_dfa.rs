use regex::Regex;

pub fn matches_hã(text: &str) -> bool {
    let pattern = r"^(hão)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
