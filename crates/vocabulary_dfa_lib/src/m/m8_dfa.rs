use regex::Regex;

pub fn matches_m8(text: &str) -> bool {
    let pattern = r"^(m8|m80|m81|m85|m88)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
