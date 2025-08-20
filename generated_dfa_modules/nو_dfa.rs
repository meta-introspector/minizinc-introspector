use regex::Regex;

pub fn matches_nو(text: &str) -> bool {
    let pattern = r"^(nوقار)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
