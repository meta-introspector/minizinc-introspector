use regex::Regex;

pub fn matches_u6(text: &str) -> bool {
    let pattern = r"^(u625|u64_bytes|u64_code|u64_collector|u64_ff|u64_field|u64_legacy_const_max|u64_legacy_const_min|u64_legacy_fn_max_value|u64_legacy_fn_min_value|u64_legacy_mod|u64_monotonic|u64_path|u64_ref|u64_some_if_not_zero|u64_suffixed|u64_to_binary|u64_to_hex|u64_to_octal|u64_unsuffixed|u64_values|u64be|u64le|u64merge|u64parser|u64tou8)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
