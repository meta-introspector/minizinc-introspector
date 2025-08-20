use regex::Regex;

pub fn matches_nこ(text: &str) -> bool {
    let pattern = r"^(nこんにちは)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
