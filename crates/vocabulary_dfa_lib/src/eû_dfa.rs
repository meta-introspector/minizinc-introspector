use regex::Regex;

pub fn matches_eû(text: &str) -> bool {
    let pattern = r"^(eûmes|eût|eûtes)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
