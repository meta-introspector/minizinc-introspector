use regex::Regex;

pub fn matches_dj(text: &str) -> bool {
    let pattern = r"^(dj|djc|djcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
