use regex::Regex;

pub fn matches_g9(text: &str) -> bool {
    let pattern = r"^(g9lsdj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
