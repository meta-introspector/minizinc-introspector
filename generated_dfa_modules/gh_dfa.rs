use regex::Regex;

pub fn matches_gh(text: &str) -> bool {
    let pattern = r"^(gh_benches|gh_copilot_token|gh_index_benchmark|gh_logs|gh_response|gh_small|gh_status|ghana|ghcr|gheqhl1q|ghiabcdef|ghiabjklcdef|ghiabjlcdef|ghiamnoef|ghijkl|ghissuecontext|ghost_data_graphs|ghost_element|ghostelementactive|ghostelementbackground|ghostelementdisabled|ghostelementhover|ghostelementselected|ghostty|ghp_1234567890|ghsotwfmh6xurrljcxcx62h7748n2uq8mf87hugkmphg|ghz|ghˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
