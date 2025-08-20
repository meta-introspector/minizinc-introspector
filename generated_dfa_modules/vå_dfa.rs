use regex::Regex;

pub fn matches_vå(text: &str) -> bool {
    let pattern = r"^(vår|våra|vårt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
