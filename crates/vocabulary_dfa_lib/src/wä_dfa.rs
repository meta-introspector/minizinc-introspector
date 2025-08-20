use regex::Regex;

pub fn matches_wä(text: &str) -> bool {
    let pattern = r"^(während)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
