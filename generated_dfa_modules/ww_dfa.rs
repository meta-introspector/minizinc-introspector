use regex::Regex;

pub fn matches_ww(text: &str) -> bool {
    let pattern = r"^(ww|ww_negated|wwarnings|wwb6qu867aw4|www3|www_authenticate|www_negated)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
