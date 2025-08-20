use regex::Regex;

pub fn matches_w7(text: &str) -> bool {
    let pattern = r"^(w7|w72b|w7btwin2t)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
