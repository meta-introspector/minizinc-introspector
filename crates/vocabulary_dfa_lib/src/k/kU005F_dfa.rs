use regex::Regex;

pub fn matches_k_(text: &str) -> bool {
    let pattern = r"^(k_b|k_batch_stride|k_bias|k_block_size|k_buffer|k_cache|k_cache_mut|k_cont|k_coords|k_current|k_diffusion|k_dims|k_embed|k_flat|k_for_ty|k_head|k_head_stride|k_i|k_ids|k_idx|k_in_lhs_blocks|k_in_rhs_blocks|k_layernorm|k_layout|k_len|k_lin|k_means_bench|k_means_incr_bench|k_means_init|k_means_init_bench|k_means_model|k_means_para|k_means_plusplus|k_nearest_bench|k_nope|k_norm|k_offset|k_padded|k_pass|k_path|k_pe|k_proj|k_ptr|k_q|k_q_masked|k_q_scaled|k_q_scaled_alibi|k_q_soft_max|k_q_v|k_q_v_merged|k_rank|k_rot|k_row_stride|k_s0|k_s1|k_s2|k_s3|k_seq_len|k_shuffle|k_size_c|k_sizes_and_dilations|k_state|k_str|k_stride|k_t|k_target_len|k_token|k_top|k_top_nodes|k_type|k_x|k_y)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
