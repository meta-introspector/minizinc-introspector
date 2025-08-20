use regex::Regex;

pub fn matches_pf(text: &str) -> bool {
    let pattern = r"^(pf_local_with_inferred_type_issue7053|pfambuyti92nhfm|pfcallback|pfeil|pfields|pfl|pfq4ad4w|pfr|pfx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
