use regex::Regex;

pub fn matches_wˇ(text: &str) -> bool {
    let pattern = r"^(wˇord)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
