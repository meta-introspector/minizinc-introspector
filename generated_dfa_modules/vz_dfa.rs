use regex::Regex;

pub fn matches_vz(text: &str) -> bool {
    let pattern = r"^(vzeroonepointfivemedium|vzi|vzig|vzigz|vzigza|vzigzag)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
