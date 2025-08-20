use regex::Regex;

pub fn matches_wg(text: &str) -> bool {
    let pattern = r"^(wgslquantizedtype|wgt_descriptor)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
