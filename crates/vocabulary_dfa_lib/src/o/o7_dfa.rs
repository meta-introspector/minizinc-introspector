use regex::Regex;

pub fn matches_o7(text: &str) -> bool {
    let pattern = r"^(o777)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
