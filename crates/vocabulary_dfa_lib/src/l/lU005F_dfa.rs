use regex::Regex;

pub fn matches_l_(text: &str) -> bool {
    let pattern = r"^(l_|l_0|l_2|l_____e|l_abi|l_amount|l_an|l_args|l_arr|l_attn|l_bin|l_brack_token|l_broadcast|l_bros|l_bytes|l_case|l_cond|l_container|l_cos|l_curly_range|l_curlyt|l_delimiter|l_elem|l_expr|l_extra_checks|l_f_exp|l_f_ident|l_fez|l_field|l_fields|l_fun|l_i|l_id|l_ident|l_in_i|l_inc|l_inf|l_info|l_key|l_last|l_len|l_lower|l_lt|l_map|l_mut|l_name|l_node|l_nonexh|l_num|l_o_l|l_op|l_out_i|l_p|l_pad|l_path|l_pid|l_poly|l_prefix|l_pubkey|l_qux|l_range|l_receiver|l_rmut|l_scales|l_shape|l_shift_dir|l_sin|l_span|l_src|l_stake|l_str|l_sysid|l_tup|l_type|l_vec|l_whence)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
