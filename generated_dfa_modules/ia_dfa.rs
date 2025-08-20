use regex::Regex;

pub fn matches_ia(text: &str) -> bool {
    let pattern = r"^(ia802301|ia_i64|ia_i64_bf16|ia_i64_f16|ia_i64_f32|ia_i64_i64|ia_i64_u32|ia_i64_u8|ia_u32|ia_u32_bf16|ia_u32_f16|ia_u32_f32|ia_u32_i64|ia_u32_u32|ia_u32_u8|ia_u8|ia_u8_bf16|ia_u8_f16|ia_u8_f32|ia_u8_i64|ia_u8_u32|ia_u8_u8|iaas|iac|iacr|iacu|iaculis|iacut|iacute|iadd|iadd_imm|ializing|iauthority)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
