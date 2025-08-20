use regex::Regex;

pub fn matches_dé(text: &str) -> bool {
    let pattern = r"^(dé1|décisives|dédale|déjà_vu_cairo|désagréable|détient)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
