use regex::Regex;

pub fn matches_fy(text: &str) -> bool {
    let pattern = r"^(fy1|fy2|fy3)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
