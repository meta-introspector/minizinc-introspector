use regex::Regex;

pub fn matches_i5(text: &str) -> bool {
    let pattern = r"^(i5gavcfsqolzfzvh0dlcguq1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
