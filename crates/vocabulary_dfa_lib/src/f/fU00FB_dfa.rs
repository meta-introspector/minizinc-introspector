use regex::Regex;

pub fn matches_fû(text: &str) -> bool {
    let pattern = r"^(fûmes|fûtes)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
