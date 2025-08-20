use regex::Regex;

pub fn matches_wü(text: &str) -> bool {
    let pattern = r"^(würde|würden)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
