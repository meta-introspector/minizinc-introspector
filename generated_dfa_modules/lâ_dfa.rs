use regex::Regex;

pub fn matches_lâ(text: &str) -> bool {
    let pattern = r"^(lâché)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
