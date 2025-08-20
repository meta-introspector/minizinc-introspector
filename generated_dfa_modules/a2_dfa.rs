use regex::Regex;

pub fn matches_a2(text: &str) -> bool {
    let pattern = r"^(a256gcm|a2a_method|a2a_type|a2arequest|a2aresponse|a2n)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
