use regex::Regex;

pub fn matches_qx(text: &str) -> bool {
    let pattern = r"^(qxqk0wwo|qxzzotxiwlhusuhaxjudmiv3fjq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
