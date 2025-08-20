use regex::Regex;

pub fn matches_mg(text: &str) -> bool {
    let pattern = r"^(mgechev|mglyph|mgmt|mgmtmethod|mgr|mgresult|mgsloan)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
