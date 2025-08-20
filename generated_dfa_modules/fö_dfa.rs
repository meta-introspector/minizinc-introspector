use regex::Regex;

pub fn matches_fö(text: &str) -> bool {
    let pattern = r"^(för)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
