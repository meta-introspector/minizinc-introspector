use regex::Regex;

pub fn matches_má(text: &str) -> bool {
    let pattern = r"^(már|más|másik)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
