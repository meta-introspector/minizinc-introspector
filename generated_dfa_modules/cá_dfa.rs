use regex::Regex;

pub fn matches_cá(text: &str) -> bool {
    let pattern = r"^(cáscara)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
