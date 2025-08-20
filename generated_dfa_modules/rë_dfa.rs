use regex::Regex;

pub fn matches_rë(text: &str) -> bool {
    let pattern = r"^(rëevaluate)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
