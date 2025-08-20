use regex::Regex;

pub fn matches_dä(text: &str) -> bool {
    let pattern = r"^(där)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
