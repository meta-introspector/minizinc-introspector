use regex::Regex;

pub fn matches_qq(text: &str) -> bool {
    let pattern = r"^(qq_depth|qqqqq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
