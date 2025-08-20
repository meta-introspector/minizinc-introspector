use regex::Regex;

pub fn matches_ux(text: &str) -> bool {
    let pattern = r"^(ux2b|uxes|uxguide)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
