use regex::Regex;

pub fn matches_aあ(text: &str) -> bool {
    let pattern = r"^(aあ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
