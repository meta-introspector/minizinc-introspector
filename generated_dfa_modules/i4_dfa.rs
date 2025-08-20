use regex::Regex;

pub fn matches_i4(text: &str) -> bool {
    let pattern = r"^(i4|i400|i420|i420_buffer|i420_to_nv12)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
