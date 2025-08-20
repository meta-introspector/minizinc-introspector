use regex::Regex;

pub fn matches_dˇ(text: &str) -> bool {
    let pattern = r"^(dˇ|dˇef|dˇoes_not_exist)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
