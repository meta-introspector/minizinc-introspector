use regex::Regex;

pub fn matches_s2(text: &str) -> bool {
    let pattern = r"^(s20|s24|s24khz|s256|s2_|s2_addr|s2_idx|s2_matches|s2d_cache|s2p|s2s)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
