use regex::Regex;

pub fn matches_n7(text: &str) -> bool {
    let pattern = r"^(n7|n777|n7777|n789abcδf01234567)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
