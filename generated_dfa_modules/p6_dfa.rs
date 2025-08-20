use regex::Regex;

pub fn matches_p6(text: &str) -> bool {
    let pattern = r"^(p622_bahmanbahmani_vldb2012|p625|p64|p6760)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
