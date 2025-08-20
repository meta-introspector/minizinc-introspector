use regex::Regex;

pub fn matches_l0(text: &str) -> bool {
    let pattern = r"^(l00|l01|l0_plus_l1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
