use regex::Regex;

pub fn matches_a9(text: &str) -> bool {
    let pattern = r"^(a9lo_parade_|a9lorution_|a9m)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
