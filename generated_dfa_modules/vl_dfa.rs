use regex::Regex;

pub fn matches_vl(text: &str) -> bool {
    let pattern = r"^(vlaanderen|vladbat00|vladimir|vlan0|vlas|vld1_u32|vld1_u8|vld1q_f32|vld1q_s16|vld1q_s16_x2|vld1q_s8|vld1q_s8_x2|vld1q_s8_x4|vld1q_u8|vld1q_u8_x2|vld1q_u8_x4|vldb|vlen|vlink|vlm|vlms|vlt|vltr|vltri|vlx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
