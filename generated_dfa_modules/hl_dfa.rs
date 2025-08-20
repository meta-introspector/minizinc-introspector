use regex::Regex;

pub fn matches_hl(text: &str) -> bool {
    let pattern = r"^(hl_config|hl_exit_points|hl_ranges|hljs|hljyffa4x|hllolo|hlo|hloperator|hlp|hlpunct|hls|hlsa|hlt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
