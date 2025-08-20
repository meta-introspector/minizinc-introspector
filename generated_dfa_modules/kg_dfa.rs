use regex::Regex;

pub fn matches_kg(text: &str) -> bool {
    let pattern = r"^(kglex|kgoar|kgr|kgre|kgree|kgreen)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
