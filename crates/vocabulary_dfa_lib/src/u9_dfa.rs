use regex::Regex;

pub fn matches_u9(text: &str) -> bool {
    let pattern = r"^(u9|u96|u96hdtcx4q7gub)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
