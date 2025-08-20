use regex::Regex;

pub fn matches_lg(text: &str) -> bool {
    let pattern = r"^(lg_i|lg_n|lg_norm|lgbt|lgdi32|lge)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
