use regex::Regex;

pub fn matches_uy(text: &str) -> bool {
    let pattern = r"^(uy|uyniwqamlkmo9tcdqzmyvhl8fv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
