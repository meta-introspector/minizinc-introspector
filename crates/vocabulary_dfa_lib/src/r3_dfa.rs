use regex::Regex;

pub fn matches_r3(text: &str) -> bool {
    let pattern = r"^(r30|r31|r3r2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
