use regex::Regex;

pub fn matches_n0(text: &str) -> bool {
    let pattern = r"^(n000|n0x0|n0x0020|n0x10000)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
