use regex::Regex;

pub fn matches_qg(text: &str) -> bool {
    let pattern = r"^(qgrams)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
