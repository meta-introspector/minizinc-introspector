use regex::Regex;

pub fn matches_wø(text: &str) -> bool {
    let pattern = r"^(wørld)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
