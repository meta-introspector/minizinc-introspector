use regex::Regex;

pub fn matches_té(text: &str) -> bool {
    let pattern = r"^(tém|témoin)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
