use regex::Regex;

pub fn matches_só(text: &str) -> bool {
    let pattern = r"^(só)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
