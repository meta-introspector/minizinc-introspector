use regex::Regex;

pub fn matches_f_(text: &str) -> bool {
    let pattern = r"^(f_|f_a|f_apb|f_arg|f_arg_is_some|f_args|f_ast_id|f_b|f_box_t|f_by_ref|f_by_value|f_c|f_cache_path|f_content|f_d|f_def|f_flag|f_function|f_getfl|f_getpath|f_grad|f_hashmap|f_i|f_in_trait|f_kinfo|f_n|f_names|f_owned|f_p|f_path|f_rdlck|f_recv|f_ref|f_ref_t|f_score|f_setfl|f_setlk|f_setlkw|f_species_id_mapping|f_str|f_str_t|f_string|f_sum_grad|f_t|f_to_x|f_txt|f_type|f_unlck|f_untyped|f_used_once|f_value|f_vec|f_wrlck)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
