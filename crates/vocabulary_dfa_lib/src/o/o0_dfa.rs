use regex::Regex;

pub fn matches_o0(text: &str) -> bool {
    let pattern = r"^(o0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
