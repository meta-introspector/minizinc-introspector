use regex::Regex;

pub fn matches_r7(text: &str) -> bool {
    let pattern = r"^(r7|r7rs_benchmark_test_suite|r7rs_test_suite)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
