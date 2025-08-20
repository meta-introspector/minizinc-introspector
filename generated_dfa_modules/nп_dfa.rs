use regex::Regex;

pub fn matches_nп(text: &str) -> bool {
    let pattern = r"^(nпривет)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
