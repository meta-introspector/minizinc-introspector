use regex::Regex;

pub fn matches_vx(text: &str) -> bool {
    let pattern = r"^(vxworks)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
