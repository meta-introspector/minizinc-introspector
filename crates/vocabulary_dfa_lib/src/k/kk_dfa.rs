use regex::Regex;

pub fn matches_kk(text: &str) -> bool {
    let pattern = r"^(kkkk|kkt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
