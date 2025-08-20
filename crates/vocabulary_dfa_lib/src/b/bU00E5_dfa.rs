use regex::Regex;

pub fn matches_bå(text: &str) -> bool {
    let pattern = r"^(både|båe)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
