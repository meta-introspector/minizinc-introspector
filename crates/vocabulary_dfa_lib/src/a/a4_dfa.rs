use regex::Regex;

pub fn matches_a4(text: &str) -> bool {
    let pattern = r"^(a4_freq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
