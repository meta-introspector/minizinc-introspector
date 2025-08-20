use regex::Regex;

pub fn matches_aˇ(text: &str) -> bool {
    let pattern = r"^(aˇaaˇbaaa|aˇaˇaaa|aˇb|aˇbcd|aˇbˇ|aˇbˇc|aˇc|aˇello|aˇx|aˇßcdˇe|aˇαˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
