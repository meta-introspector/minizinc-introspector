use regex::Regex;

pub fn matches_uj(text: &str) -> bool {
    let pattern = r"^(uj|ujsw9xqblgkvhav)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
