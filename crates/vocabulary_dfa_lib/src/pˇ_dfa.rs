use regex::Regex;

pub fn matches_pˇ(text: &str) -> bool {
    let pattern = r"^(pˇ|pˇfield|pˇrintln)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
