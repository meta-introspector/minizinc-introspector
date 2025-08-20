use regex::Regex;

pub fn matches_o9(text: &str) -> bool {
    let pattern = r"^(o95e)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
