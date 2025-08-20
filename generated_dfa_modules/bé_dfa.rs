use regex::Regex;

pub fn matches_bé(text: &str) -> bool {
    let pattern = r"^(bébé|bézier)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
