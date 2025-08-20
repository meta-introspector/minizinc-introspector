use regex::Regex;

pub fn matches_wj(text: &str) -> bool {
    let pattern = r"^(wjalrxutnfemi)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
