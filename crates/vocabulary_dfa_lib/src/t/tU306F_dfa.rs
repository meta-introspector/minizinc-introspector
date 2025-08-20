use regex::Regex;

pub fn matches_tは(text: &str) -> bool {
    let pattern = r"^(tは)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
