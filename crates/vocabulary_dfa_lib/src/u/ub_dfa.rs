use regex::Regex;

pub fn matches_ub(text: &str) -> bool {
    let pattern = r"^(ub1|ub2|ub6ssrre|ub_cfg|ub_checks|ubb_enable|ubchecks|uber|ubinary|ubiquous|ubr|ubrc|ubrcy|ubre|ubrev|ubreve|ubs|ubsequent)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
