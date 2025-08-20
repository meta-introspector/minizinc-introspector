use regex::Regex;

pub fn matches_i1(text: &str) -> bool {
    let pattern = r"^(i128_legacy_const_max|i128_legacy_const_min|i128_legacy_fn_max_value|i128_legacy_fn_min_value|i128_legacy_mod|i128_max|i128_path|i128_to_binary|i128_to_hex|i128_to_octal|i128be|i128le|i128parser|i16_legacy_const_max|i16_legacy_const_min|i16_legacy_fn_max_value|i16_legacy_fn_min_value|i16_legacy_mod|i16_path|i16_repr_a|i16_suffixed|i16_to_binary|i16_to_hex|i16_to_octal|i16be|i16frames|i16i64|i16le|i16parser|i16x8|i16x8_add|i16x8_extend_high_i8x16|i16x8_extend_high_u8x16|i16x8_extend_low_i8x16|i16x8_extend_low_u8x16|i16x8_load_extend_i8x8|i16x8_load_extend_u8x8|i16x8_mul|i16x8_shr|i16x8_splat|i16x8_sub|i18n_embed|i18n_embed_fl|i18ndatatype|i1a|i1b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
