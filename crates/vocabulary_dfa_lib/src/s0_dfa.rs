use regex::Regex;

pub fn matches_s0(text: &str) -> bool {
    let pattern = r"^(s0022000003000254|s02|s0956796812000093|s0_str|s0k0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
