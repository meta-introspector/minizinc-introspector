use regex::Regex;

pub fn matches_qb(text: &str) -> bool {
    let pattern = r"^(qbynqiataibaaaabnoc3smwt4)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
