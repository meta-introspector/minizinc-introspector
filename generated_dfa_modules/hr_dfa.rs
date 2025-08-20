use regex::Regex;

pub fn matches_hr(text: &str) -> bool {
    let pattern = r"^(hr1nua9b7nj6echs26o7vi8gyyddwwd3yebfzjktbu86|hrange|hree|hreflang|hresult|hrl|hrtb_bound_does_not_add_dyn|hrtb_lifetimes)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
