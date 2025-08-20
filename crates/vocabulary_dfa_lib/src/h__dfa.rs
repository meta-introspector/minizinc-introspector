use regex::Regex;

pub fn matches_h_(text: &str) -> bool {
    let pattern = r"^(h_0|h_0p5|h_10|h_11|h_12|h_128|h_16|h_1_3|h_24|h_2_3|h_2_5|h_32|h_40|h_64|h_72|h___|h____y|h_acc|h_auto|h_bytes|h_cursor|h_factors|h_group|h_group_lg|h_group_sm|h_group_xl|h_i|h_ij|h_in|h_k_idx|h_l|h_list|h_module|h_name|h_out|h_p|h_phi_1|h_phi_k|h_r|h_ratio|h_result|h_scales|h_scroll_offset|h_stride|h_times_b_div_s|h_times_d|h_trigger|h_value|h_vec)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
