use regex::Regex;

pub fn matches_nを(text: &str) -> bool {
    let pattern = r"^(nを用いる)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
