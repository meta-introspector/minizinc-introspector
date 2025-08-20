use regex::Regex;

pub fn matches_vv(text: &str) -> bool {
    let pattern = r"^(vv_prints_rustc_env_vars|vv_prints_warnings|vv_prints_warnings_git|vv_prints_warnings_http|vvcos|vvcosf|vvd|vvda|vvdas|vvdash|vvexp|vvexpf|vvlog|vvlogf|vvsin|vvsinf|vvsqrt|vvsqrtf|vvtanh|vvtanhf|vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
