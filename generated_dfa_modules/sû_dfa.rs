use regex::Regex;

pub fn matches_sû(text: &str) -> bool {
    let pattern = r"^(sûrs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
