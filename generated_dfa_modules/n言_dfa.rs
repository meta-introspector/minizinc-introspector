use regex::Regex;

pub fn matches_n言(text: &str) -> bool {
    let pattern = r"^(n言語)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
