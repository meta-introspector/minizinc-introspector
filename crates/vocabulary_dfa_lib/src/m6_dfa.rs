use regex::Regex;

pub fn matches_m6(text: &str) -> bool {
    let pattern = r"^(m60|m61|m65|m667|m68k|m6ncoikjiwdcwqkh5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
