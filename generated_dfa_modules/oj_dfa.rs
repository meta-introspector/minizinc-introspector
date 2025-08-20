use regex::Regex;

pub fn matches_oj(text: &str) -> bool {
    let pattern = r"^(ojek4e8jhjtlt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
