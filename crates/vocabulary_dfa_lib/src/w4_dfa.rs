use regex::Regex;

pub fn matches_w4(text: &str) -> bool {
    let pattern = r"^(w4|w4b|w4yce6gbodqntwlg7)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
