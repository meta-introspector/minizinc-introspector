use regex::Regex;

pub fn matches_v0(text: &str) -> bool {
    let pattern = r"^(v0_0|v0_0_3|v0_0_6|v0_0h|v0_0hs|v0_0l|v0_0ls|v0_1_0|v0_1_5|v0_1_8|v0_2_0|v0_2_8|v0_3_0|v0_4_0|v0_5_0|v0_6_0|v0_message|v0_msg|v0_with_multiple_lookups|v0_with_single_lookup)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
