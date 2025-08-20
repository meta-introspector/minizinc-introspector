use regex::Regex;

pub fn matches_i8(text: &str) -> bool {
    let pattern = r"^(i80|i80d9s77ihctr8xlfi|i8_legacy_const_max|i8_legacy_const_min|i8_legacy_fn_max_value|i8_legacy_fn_min_value|i8_legacy_mod|i8_path|i8_ptr|i8_repr_a|i8_repr_b|i8_repr_c|i8_repr_d|i8_suffixed|i8_to_binary|i8_to_hex|i8_to_octal|i8_to_u8|i8mm|i8parser|i8x16_narrow_i16x8|i8x16_splat|i8x16_sub)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
