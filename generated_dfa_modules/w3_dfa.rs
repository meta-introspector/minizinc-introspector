use regex::Regex;

pub fn matches_w3(text: &str) -> bool {
    let pattern = r"^(w360m|w3_0_6b|w3_14b|w3_1_7b|w3_32b|w3_4b|w3_8b|w3id|w3moea3b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
