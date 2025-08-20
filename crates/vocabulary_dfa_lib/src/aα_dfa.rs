use regex::Regex;

pub fn matches_aα(text: &str) -> bool {
    let pattern = r"^(aαbβ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
