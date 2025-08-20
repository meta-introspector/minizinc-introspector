use regex::Regex;

pub fn matches_oˇ(text: &str) -> bool {
    let pattern = r"^(oˇe|oˇne|oˇover)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
