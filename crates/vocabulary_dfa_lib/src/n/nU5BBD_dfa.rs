use regex::Regex;

pub fn matches_n宽(text: &str) -> bool {
    let pattern = r"^(n宽3456)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
