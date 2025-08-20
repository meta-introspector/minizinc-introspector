use regex::Regex;

pub fn matches_iy(text: &str) -> bool {
    let pattern = r"^(iy1|iy2|iy3)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
