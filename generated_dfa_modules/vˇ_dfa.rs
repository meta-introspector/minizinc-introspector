use regex::Regex;

pub fn matches_vˇ(text: &str) -> bool {
    let pattern = r"^(vˇariable)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
