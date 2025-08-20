use regex::Regex;

pub fn matches_né(text: &str) -> bool {
    let pattern = r"^(né|néha|néhány|nélkül)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
