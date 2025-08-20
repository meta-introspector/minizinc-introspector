use regex::Regex;

pub fn matches_w0(text: &str) -> bool {
    let pattern = r"^(w00t|w0_5b|w0_span)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
