use regex::Regex;

pub fn matches_kö(text: &str) -> bool {
    let pattern = r"^(können|könnte|között|közül)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
