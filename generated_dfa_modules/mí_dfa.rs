use regex::Regex;

pub fn matches_mí(text: &str) -> bool {
    let pattern = r"^(mí|mía|mías|míg|mío|míos)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
