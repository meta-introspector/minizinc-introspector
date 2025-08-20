use regex::Regex;

pub fn matches_p_(text: &str) -> bool {
    let pattern = r"^(p_|p_1_3|p______d|p_arr|p_auditor|p_base|p_cell|p_credential|p_default|p_desc|p_dest|p_destination|p_expr|p_f64|p_file|p_first|p_float|p_foo|p_foo2|p_h|p_idx|p_in|p_is_zeros|p_j|p_k|p_key_states|p_kind|p_left|p_lt|p_m|p_m1|p_match|p_mut|p_name|p_new_info_opt|p_node|p_noise|p_out|p_out_of_bounds|p_patch|p_path|p_px|p_required_env_opt|p_required_states_opt|p_right|p_rw|p_satlebel_memo_opt|p_second|p_seg|p_snodes|p_source|p_sz|p_term|p_third|p_token|p_tokens|p_ty|p_type|p_value_states|p_values|p_w)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
