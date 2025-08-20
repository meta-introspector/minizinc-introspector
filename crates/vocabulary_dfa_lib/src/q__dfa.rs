use regex::Regex;

pub fn matches_q_(text: &str) -> bool {
    let pattern = r"^(q_a_layernorm|q_a_proj|q_b|q_b_proj|q_batch_stride|q_bias|q_buffer|q_bundle_metadata_path|q_ci|q_cli|q_cli_default|q_codespaces|q_coords|q_current|q_debug_shell|q_dev_bext|q_developer_standalone|q_developer_standalone_free|q_developer_standalone_power|q_developer_standalone_pro|q_developer_standalone_pro_plus|q_dim|q_dims_for_matmul|q_disable_telemetry|q_disable_truecolor|q_embed|q_end|q_fake_is_remote|q_fixed|q_flat|q_h|q_head|q_head_dim|q_head_stride|q_ids|q_ij|q_key|q_l|q_layernorm|q_length|q_lin|q_log_level_global|q_log_stdout|q_lora_rank|q_mat_mul_compute_node_key|q_mat_mul_dependencies|q_matrix|q_matrix_metadata|q_mock_chat_response|q_new|q_noise|q_nope|q_norm|q_offset|q_parent|q_pass|q_path|q_pe|q_prompt_|q_ptr|q_rank|q_rot|q_row_stride|q_seq|q_set_parent|q_set_parent_check|q_shape|q_shell|q_silences_warnings|q_size|q_split|q_start|q_subscription_type|q_sz|q_target|q_telemetry_client_id|q_term|q_using_zsh_autosuggestions|q_zdotdir)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
