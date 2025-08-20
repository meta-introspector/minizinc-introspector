use regex::Regex;

pub fn matches_b6(text: &str) -> bool {
    let pattern = r"^(b64_account|b64_host|b64_salt|b64url)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
