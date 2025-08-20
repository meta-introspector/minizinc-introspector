use regex::Regex;

pub fn matches_n笔(text: &str) -> bool {
    let pattern = r"^(n笔)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
