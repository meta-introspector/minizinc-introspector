use regex::Regex;

pub fn matches_kb(text: &str) -> bool {
    let pattern = r"^(kb_size|kbgqwcqqtytbwhx3bz5vwtfdlhrn0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
