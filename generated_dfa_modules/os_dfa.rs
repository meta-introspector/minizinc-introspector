use regex::Regex;

pub fn matches_os(text: &str) -> bool {
    let pattern = r"^(os_architecture|os_code|os_env|os_environment|os_info|os_information|os_instructions|os_keymap|os_manager|os_memory_stats_reporting|os_memory_stats_reporting_arg|os_release|os_release_parse|os_release_str|os_secret|os_shim_internal|os_specific_instructions|os_state|os_str_err|os_str_slice|os_str_to_os_string|os_string_as_os_str|os_string_pathbuf_leak|os_string_truncate|os_suffix|osaka|oscillates|oscillating|oscilloscope|oscr|osdefault|osenv|osf2f|osi_layer_array|osi_layers|osilayer3|osilayer4|osilayer5|osilayer6|osilayer7|osl|osla|oslas|oslash|osmfoundation|osol|osp|ospg|osrelease|osseus|osstrerr|ostrich|osv|osvs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
