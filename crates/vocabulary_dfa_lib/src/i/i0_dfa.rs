use regex::Regex;

pub fn matches_i0(text: &str) -> bool {
    let pattern = r"^(i01)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
