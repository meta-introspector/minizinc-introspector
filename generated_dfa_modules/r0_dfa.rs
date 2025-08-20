use regex::Regex;

pub fn matches_r0(text: &str) -> bool {
    let pattern = r"^(r00|r01|r0_3_0|r0_3_1|r0_3_2|r0_3_3|r0_3_4|r0_4_0|r0_4_1|r0_4_2|r0_4_3|r0_5_0|r0_5_1|r0_5_2|r0_6_0|r0_6_1|r0_6_2|r0_6_3|r0_6_4|r0_6_5|r0_6_6|r0_6_7|r0_7_0|r0_7_1|r0_7_2|r0_7_3|r0_7_4|r0_8_0|r0_8_1|r0_8_2|r0_8_3|r0_8_4|r0_8_7|r0_8_8|r0_plus_r1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
