use regex::Regex;

pub fn matches_bˇ(text: &str) -> bool {
    let pattern = r"^(bˇ|bˇa|bˇb|bˇbbb|bˇbbˇb|bˇn|bˇody|bˇr|bˇrown)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
