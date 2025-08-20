use regex::Regex;

pub fn matches_i3(text: &str) -> bool {
    let pattern = r"^(i32_as_literal|i32_const_token|i32_field|i32_legacy_const_max|i32_legacy_const_min|i32_legacy_fn_max_value|i32_legacy_fn_min_value|i32_legacy_mod|i32_path|i32_ref|i32_suffixed|i32_to_binary|i32_to_hex|i32_to_octal|i32_to_usize|i32_ty|i32_unsuffixed|i32alias|i32be|i32le|i32parser|i32simd|i32visitor|i32x4_add|i32x4_dot_i16x8|i32x4_extend_high_i16x8|i32x4_extend_low_i16x8|i32x4_load_extend_i16x4|i32x4_mul|i32x4_shr|i32x4_splat)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
