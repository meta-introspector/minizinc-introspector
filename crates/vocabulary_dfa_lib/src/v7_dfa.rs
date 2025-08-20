use regex::Regex;

pub fn matches_v7(text: &str) -> bool {
    let pattern = r"^(v7_0|v7_checks|v7_debug_test_results|v7tests)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
