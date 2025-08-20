use regex::Regex;

pub fn matches_o4(text: &str) -> bool {
    let pattern = r"^(o4_mini|o4_mini_2025_04_16|o4mini|o4qxx719uczywl50tb5tdqpgmj54jcq3vuiu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
