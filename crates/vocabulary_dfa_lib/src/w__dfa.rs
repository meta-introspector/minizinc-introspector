use regex::Regex;

pub fn matches_w_(text: &str) -> bool {
    let pattern = r"^(w_|w_112|w_128|w_1_3|w_1_4|w_2|w_2_4|w_2_5|w_3|w_3_4|w_3_5|w_40|w_48|w_4_5|w_6|w_64|w____|w_area|w_auto|w_b_minus|w_b_plus|w_b_zero|w_bank_forks|w_blockhash_queue|w_bn|w_gen|w_hh|w_hh_init|w_highest_primary_index_slot|w_ih|w_ih_init|w_in|w_in_proj|w_index|w_k_idx|w_key|w_last_notified_slot|w_last_unnotified_slot|w_maps|w_max|w_maybe_unflushed_roots|w_norm2|w_ongoing_scan_roots|w_optimistically_confirmed_bank|w_out|w_outer_keys|w_p|w_prefix|w_query|w_ratio|w_replay_progress|w_replay_stats|w_roots_tracker|w_slot_tracker|w_slot_vote_tracker|w_slot_vote_trackers|w_some|w_stride|w_sum|w_t|w_token|w_value)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
