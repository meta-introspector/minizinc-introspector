use regex::Regex;

pub fn matches_ik(text: &str) -> bool {
    let pattern = r"^(ikke|ikkje)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
