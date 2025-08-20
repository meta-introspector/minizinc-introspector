use regex::Regex;

pub fn matches_qh(text: &str) -> bool {
    let pattern = r"^(qhbits|qhead|qhq7ppvg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
