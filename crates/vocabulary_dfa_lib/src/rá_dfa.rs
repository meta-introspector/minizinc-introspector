use regex::Regex;

pub fn matches_rá(text: &str) -> bool {
    let pattern = r"^(rá|rámon|rápido)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
