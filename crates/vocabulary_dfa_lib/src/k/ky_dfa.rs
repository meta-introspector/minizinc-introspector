use regex::Regex;

pub fn matches_ky(text: &str) -> bool {
    let pattern = r"^(ky_fan_norm|kylobytes|kytea_dict|kytea_wsconst|kyteaconfig)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
