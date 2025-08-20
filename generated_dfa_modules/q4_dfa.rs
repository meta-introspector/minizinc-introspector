use regex::Regex;

pub fn matches_q4(text: &str) -> bool {
    let pattern = r"^(q40|q41|q47543030|q4_0_block_size|q4_1_with_f16|q4_2|q4_3|q4bits|q4bits1|q4bits2|q4bits_h|q4bytes|q4h|q4h_0|q4h_1|q4h_2|q4h_3|q4l|q4scale)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
