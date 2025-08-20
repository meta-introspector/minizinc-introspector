use regex::Regex;

pub fn matches_iw(text: &str) -> bool {
    let pattern = r"^(iw|iwicimagingfactory)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
