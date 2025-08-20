use regex::Regex;

pub fn matches_ry(text: &str) -> bool {
    let pattern = r"^(ryjl3|ryme|ryu|ryu_js|ryū)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
