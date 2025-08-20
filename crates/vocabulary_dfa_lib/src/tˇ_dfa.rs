use regex::Regex;

pub fn matches_tˇ(text: &str) -> bool {
    let pattern = r"^(tˇest|tˇhello|tˇthree|tˇwelve|tˇwo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
