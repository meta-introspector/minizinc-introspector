use regex::Regex;

pub fn matches_lú(text: &str) -> bool {
    let pattern = r"^(lúthien)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
