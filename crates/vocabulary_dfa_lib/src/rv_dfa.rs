use regex::Regex;

pub fn matches_rv(text: &str) -> bool {
    let pattern = r"^(rv_a|rv_b|rvacantentry|rvalue_hint|rvalue_locals|rvaluelikeunsized|rvaluereference|rvarname|rvec_macro|rvgg_in1k|rvnetsmebhhlnmmgo9baz|rvq_first|rvq_rest|rvr|rvref|rvu|rvxzo5zyrd6ytm7x3gjplgogjjx6ptoz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
