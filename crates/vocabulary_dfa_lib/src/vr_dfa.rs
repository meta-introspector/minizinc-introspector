use regex::Regex;

pub fn matches_vr(text: &str) -> bool {
    let pattern = r"^(vram|vrd|vreg|vreinterpret_u64_u8|vreinterpret_u8_u32|vreinterpretq_s16_u16|vreinterpretq_s8_u8|vreinterpretq_u16_u8|vrsave|vrt|vrtr|vrtri)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
