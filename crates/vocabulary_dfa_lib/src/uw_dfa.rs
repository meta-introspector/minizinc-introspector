use regex::Regex;

pub fn matches_uw(text: &str) -> bool {
    let pattern = r"^(uwa|uwan|uwang|uwangl|uwangle|uwrx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
