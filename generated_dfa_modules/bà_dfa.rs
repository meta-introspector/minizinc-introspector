use regex::Regex;

pub fn matches_bà(text: &str) -> bool {
    let pattern = r"^(bàz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
