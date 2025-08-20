use regex::Regex;

pub fn matches_fô(text: &str) -> bool {
    let pattern = r"^(fôramos|fôssemos)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
