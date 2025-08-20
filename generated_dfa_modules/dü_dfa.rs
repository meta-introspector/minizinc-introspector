use regex::Regex;

pub fn matches_dü(text: &str) -> bool {
    let pattern = r"^(düzce_i̇l)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
