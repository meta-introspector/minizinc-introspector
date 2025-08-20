use regex::Regex;

pub fn matches_gˇ(text: &str) -> bool {
    let pattern = r"^(gˇoop)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
