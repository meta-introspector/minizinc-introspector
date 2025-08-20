use regex::Regex;

pub fn matches_r_(text: &str) -> bool {
    let pattern = r"^(r_|r_0|r_1|r_2|r__e|r_abi|r_account_maps|r_amount|r_an|r_args|r_arr|r_b|r_bignum|r_bin|r_block_commitment|r_block_commitment_cache|r_bn|r_brack_token|r_broadcast|r_bros|r_bytes|r_callback|r_claimed|r_cond|r_container|r_cubed|r_curly_range|r_delim|r_delimiter|r_delta|r_descendants|r_dist|r_elem|r_embed|r_end|r_expr|r_extra_checks|r_f_exp|r_f_ident|r_features|r_fee|r_fez|r_fields|r_fun|r_gate|r_h|r_i|r_id|r_ident|r_ind|r_inf|r_info|r_iter|r_key|r_linear|r_lower|r_lt|r_map|r_maxes|r_mut|r_name|r_new|r_next|r_node|r_nonexh|r_norm2|r_num|r_op|r_or_h|r_p|r_path|r_percentage|r_poly|r_prefix|r_pubkey|r_q|r_qux|r_rad|r_range|r_receiver|r_replay_progress|r_replay_stats|r_responder|r_rmut|r_sf|r_shape|r_shift_dir|r_slot_0_stores|r_slot_tracker|r_slot_vote_tracker|r_stake|r_str|r_t|r_terms|r_tup|r_type|r_val|r_val_index|r_vec|r_w|r_x|r_y)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
