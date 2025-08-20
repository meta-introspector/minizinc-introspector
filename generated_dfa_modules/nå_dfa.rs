use regex::Regex;

pub fn matches_nå(text: &str) -> bool {
    let pattern = r"^(nå|någon|något|några|når)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
