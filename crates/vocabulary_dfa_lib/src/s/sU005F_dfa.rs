use regex::Regex;

pub fn matches_s_(text: &str) -> bool {
    let pattern = r"^(s_|s____|s_____r|s_addr|s_align|s_attr|s_b|s_b_c_d_|s_bco|s_bignum|s_blinding|s_bn|s_char|s_cocci_|s_combinator|s_combinator_pipeline|s_ctx|s_curr|s_default_feature_var|s_diff|s_div|s_expand|s_expr|s_feature_var_map|s_filter_items|s_gco|s_h|s_i32|s_i32_ref|s_i32_ref_ref|s_i64|s_i64_bf16|s_i64_f16|s_i64_f32|s_i_i|s_i_inv|s_id|s_ident|s_idx|s_irgrp|s_iroth|s_irwxg|s_irwxo|s_irwxu|s_iter|s_iwgrp|s_k|s_l|s_last_index|s_match|s_n|s_offset|s_online_algorithm|s_orig|s_p|s_point|s_prev|s_q|s_r|s_rco|s_ref|s_ref2|s_responder|s_signed|s_size|s_space|s_sprite|s_str|s_suffix|s_surface|s_t|s_tab|s_temp|s_term|s_token|s_ty|s_u32|s_u32_bf16|s_u32_f16|s_u32_f32|s_u32_u32|s_u8|s_u8_bf16|s_u8_f16|s_u8_f32|s_val|s_var|s_variance|s_w)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
