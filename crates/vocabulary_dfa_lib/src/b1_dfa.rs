use regex::Regex;

pub fn matches_b1(text: &str) -> bool {
    let pattern = r"^(b160k|b1_0|b1_1|b1_area|b1_fail|b1g4|b1hegzthtfnqxyepzkesysxrjmidnqaxrzbq28gaewn8|b1ockhash1111111111111111111111111111111111)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
