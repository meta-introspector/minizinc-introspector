use regex::Regex;

pub fn matches_t0(text: &str) -> bool {
    let pattern = r"^(t00|t01|t0_actual|t0_chrono|t0_expected)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
