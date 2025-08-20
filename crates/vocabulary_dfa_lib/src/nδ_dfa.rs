use regex::Regex;

pub fn matches_nδ(text: &str) -> bool {
    let pattern = r"^(nδεζ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
