use regex::Regex;

pub fn matches_n么(text: &str) -> bool {
    let pattern = r"^(n么)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
