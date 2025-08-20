use regex::Regex;

pub fn matches_vj(text: &str) -> bool {
    let pattern = r"^(vj1|vjdpjcofku|vjlkpdhzb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
