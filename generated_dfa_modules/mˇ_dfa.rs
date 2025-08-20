use regex::Regex;

pub fn matches_mˇ(text: &str) -> bool {
    let pattern = r"^(mˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
