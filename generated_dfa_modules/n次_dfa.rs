use regex::Regex;

pub fn matches_n次(text: &str) -> bool {
    let pattern = r"^(n次の行)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
