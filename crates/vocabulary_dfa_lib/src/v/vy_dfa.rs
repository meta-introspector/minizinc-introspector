use regex::Regex;

pub fn matches_vy(text: &str) -> bool {
    let pattern = r"^(vy4b7xizeh0iwma6lmnzihcqzudvdfhq|vyi|vyper)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
