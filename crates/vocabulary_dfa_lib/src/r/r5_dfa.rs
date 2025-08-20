use regex::Regex;

pub fn matches_r5(text: &str) -> bool {
    let pattern = r"^(r5|r50k_base_singleton|r50kbase|r5rs|r5rs_test_suite)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
