use regex::Regex;

pub fn matches_ré(text: &str) -> bool {
    let pattern = r"^(récompense|résmé|résumé|résumé_москва)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
