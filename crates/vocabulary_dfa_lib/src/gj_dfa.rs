use regex::Regex;

pub fn matches_gj(text: &str) -> bool {
    let pattern = r"^(gjc|gjcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
