use regex::Regex;

pub fn matches_u8(text: &str) -> bool {
    let pattern = r"^(u80|u81|u8_between|u8_interface|u8_legacy_const_max|u8_legacy_const_min|u8_legacy_fn_max_value|u8_legacy_fn_min_value|u8_legacy_mod|u8_path|u8_ptr|u8_ref|u8_repr_a|u8_repr_b|u8_suffixed|u8_to_binary|u8_to_hex|u8_to_octal|u8_vec|u8parser|u8vec|u8vec_to_hex|u8wrapper|u8x16_shl|u8x16_shr|u8x16_splat|u8x32)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
