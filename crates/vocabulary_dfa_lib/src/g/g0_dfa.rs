use regex::Regex;

pub fn matches_g0(text: &str) -> bool {
    let pattern = r"^(g0|g0iq7g)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
