use regex::Regex;

pub fn matches_hε(text: &str) -> bool {
    let pattern = r"^(hε|hεl|hεll|hεllo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
