use regex::Regex;

pub fn matches_sˇ(text: &str) -> bool {
    let pattern = r"^(sˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
