use regex::Regex;

pub fn matches_iz(text: &str) -> bool {
    let pattern = r"^(iz|ize|ized|izv89sr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
