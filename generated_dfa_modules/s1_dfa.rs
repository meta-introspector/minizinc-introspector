use regex::Regex;

pub fn matches_s1(text: &str) -> bool {
    let pattern = r"^(s10|s11|s128|s13|s14|s15|s17|s18|s19|s1_addr|s1_idx|s1_matches|s1_str|s1ashing11111111111111111111111111111111111|s1ashs4je6wpb2kwihqnndpnridabedqyfycthhsrgv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
