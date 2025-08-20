use regex::Regex;

pub fn matches_lˇ(text: &str) -> bool {
    let pattern = r"^(lˇazy|lˇet|lˇlo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
