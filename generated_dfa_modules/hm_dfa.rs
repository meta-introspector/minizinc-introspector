use regex::Regex;

pub fn matches_hm(text: &str) -> bool {
    let pattern = r"^(hm9jw7of5i9dnrbos8pcucseoquph7jsp1rkbjnw7an4|hm_construct|hm_construct_definition|hm_construct_keywords|hm_construct_normal|hm_construct_with_duplicates|hm_contains_false|hm_contains_true|hm_empty|hm_empty_definition|hm_get|hm_get_error|hm_get_found|hm_insert|hm_insert_from_empty|hm_keys_to_vector_normal|hm_try_get_error|hm_try_get_found|hm_union|hm_union_definition|hm_values_to_vector_normal|hma2nhcdinw|hmac_length_error|hmacsha1|hmacsha256|hmacsha512|hmask_val|hme|hme_system|hml|hmmmm|hmodule|hmonitor|hmr|hms)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
