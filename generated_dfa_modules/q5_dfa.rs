use regex::Regex;

pub fn matches_q5(text: &str) -> bool {
    let pattern = r"^(q50|q51|q5_0_block_size|q5_k|q5_k_s|q5bits|q5bytes_0|q5bytes_1|q5bytes_2|q5bytes_3|q5h_0|q5h_1|q5h_2|q5h_3|q5k|q5l_0|q5l_0_right_shift|q5l_0_shift_input|q5l_1|q5l_1_right_shift|q5l_1_shift_input)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
