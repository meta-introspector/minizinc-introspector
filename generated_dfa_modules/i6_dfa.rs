use regex::Regex;

pub fn matches_i6(text: &str) -> bool {
    let pattern = r"^(i64_abs|i64_code|i64_legacy_const_max|i64_legacy_const_min|i64_legacy_fn_max_value|i64_legacy_fn_min_value|i64_legacy_mod|i64_path|i64_suffixed|i64_term|i64_to_binary|i64_to_hex|i64_to_i16|i64_to_i32|i64_to_octal|i64_to_u16|i64_to_u32|i64_unsuffixed|i64_val|i64_vec|i64alias|i64be|i64le|i64max|i64parser|i64x2|i6uqjbxjfejo2vmxlxgmlzt2n5eqnbznlwxtfssv1ubnjuf0mql)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
