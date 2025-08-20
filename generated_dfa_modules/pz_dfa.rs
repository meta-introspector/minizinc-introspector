use regex::Regex;

pub fn matches_pz(text: &str) -> bool {
    let pattern = r"^(pzero)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
