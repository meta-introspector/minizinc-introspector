use regex::Regex;

pub fn matches_væ(text: &str) -> bool {
    let pattern = r"^(være|været|vært)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
