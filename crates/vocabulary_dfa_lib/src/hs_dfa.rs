use regex::Regex;

pub fn matches_hs(text: &str) -> bool {
    let pattern = r"^(hs_b|hs_construct|hs_construct_definition|hs_construct_normal|hs_construct_with_duplicates|hs_contains_false|hs_contains_true|hs_insert|hs_insert_definition|hs_insert_from_empty|hs_keys_to_vector_normal|hsc|hscr|hsla_from_hex|hslas|hslash|hsm|hsm_key_id|hsm_pin|hsm_pkcs11_library_path|hsm_slot_index|hsms|hspace|hst|hstr|hstro|hstrok|hsuffix|hsum|hsum_float_8|hsv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
