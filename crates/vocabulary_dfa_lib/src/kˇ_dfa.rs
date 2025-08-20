use regex::Regex;

pub fn matches_kˇ(text: &str) -> bool {
    let pattern = r"^(kˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
