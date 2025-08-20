use regex::Regex;

pub fn matches_tで(text: &str) -> bool {
    let pattern = r"^(tです)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
