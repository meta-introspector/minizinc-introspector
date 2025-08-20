use regex::Regex;

pub fn matches_tí(text: &str) -> bool {
    let pattern = r"^(tínhamos)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
