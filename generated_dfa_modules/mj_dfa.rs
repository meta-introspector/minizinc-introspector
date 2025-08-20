use regex::Regex;

pub fn matches_mj(text: &str) -> bool {
    let pattern = r"^(mjs|mjsx|mjvalue)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
