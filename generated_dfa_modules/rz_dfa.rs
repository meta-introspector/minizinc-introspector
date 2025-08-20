use regex::Regex;

pub fn matches_rz(text: &str) -> bool {
    let pattern = r"^(rz|rzarg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
