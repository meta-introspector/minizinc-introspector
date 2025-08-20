use regex::Regex;

pub fn matches_u_(text: &str) -> bool {
    let pattern = r"^(u_|u_0|u_16|u_2|u_3|u_32|u_4|u_5|u_6|u_64|u_7|u__cuda_no_bfloat16_conversions__|u__cuda_no_half2_operators__|u__cuda_no_half_conversions__|u__cuda_no_half_operators__|u_arg|u_candidates|u_canonical|u_capself|u_case|u_conv|u_depth|u_i|u_idx|u_inc|u_inv|u_inv_sq|u_k|u_lg_i_sq|u_low|u_param|u_plane|u_span|u_sq|u_strlen|u_usize|u_value)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
