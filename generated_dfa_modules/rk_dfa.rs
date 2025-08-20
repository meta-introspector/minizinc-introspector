use regex::Regex;

pub fn matches_rk(text: &str) -> bool {
    let pattern = r"^(rk4_step|rkey|rks)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
