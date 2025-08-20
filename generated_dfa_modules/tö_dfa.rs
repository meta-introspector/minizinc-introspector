use regex::Regex;

pub fn matches_tö(text: &str) -> bool {
    let pattern = r"^(több)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
