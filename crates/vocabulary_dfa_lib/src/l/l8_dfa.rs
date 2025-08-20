use regex::Regex;

pub fn matches_l8(text: &str) -> bool {
    let pattern = r"^(l800|l809|l81|l810|l821|l823|l82c2|l855|l857|l86|l86c35|l86c63|l873|l877|l89|l8b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
