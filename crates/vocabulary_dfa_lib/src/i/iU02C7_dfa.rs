use regex::Regex;

pub fn matches_iˇ(text: &str) -> bool {
    let pattern = r"^(iˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
