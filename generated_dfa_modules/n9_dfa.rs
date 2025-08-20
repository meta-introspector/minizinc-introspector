use regex::Regex;

pub fn matches_n9(text: &str) -> bool {
    let pattern = r"^(n999|n9999)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
