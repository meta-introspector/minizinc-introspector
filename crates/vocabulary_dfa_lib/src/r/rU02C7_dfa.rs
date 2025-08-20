use regex::Regex;

pub fn matches_rˇ(text: &str) -> bool {
    let pattern = r"^(rˇ|rˇed|rˇs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
