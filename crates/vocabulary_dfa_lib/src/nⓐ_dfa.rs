use regex::Regex;

pub fn matches_nⓐ(text: &str) -> bool {
    let pattern = r"^(nⓐⓑⓒⓓⓔ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
