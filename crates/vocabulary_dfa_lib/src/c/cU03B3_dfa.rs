use regex::Regex;

pub fn matches_cγ(text: &str) -> bool {
    let pattern = r"^(cγdδ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
