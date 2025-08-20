use regex::Regex;

pub fn matches_v_(text: &str) -> bool {
    let pattern = r"^(v_1|v_2|v_2025_03_26|v_b|v_batch_stride|v_bf16|v_bias|v_buffer|v_cache|v_cache_mut|v_candidates|v_current|v_def|v_depth|v_doc|v_existing|v_f16|v_f32|v_f64|v_group|v_group_lg|v_group_sm|v_group_xl|v_hat|v_head_dim|v_head_stride|v_hidden|v_i32|v_i64|v_ij|v_l|v_lin|v_m|v_max|v_offset|v_old|v_outer|v_overflow|v_param|v_path|v_plane|v_prediction|v_prime|v_proj|v_ptr|v_rank|v_row_stride|v_seq_len|v_sig_lens|v_stride|v_t|v_u32|v_u8|v_value|v_w)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
