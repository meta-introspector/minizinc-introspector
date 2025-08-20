use regex::Regex;

pub fn matches_tこ(text: &str) -> bool {
    let pattern = r"^(tこれ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
