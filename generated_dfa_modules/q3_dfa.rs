use regex::Regex;

pub fn matches_q3(text: &str) -> bool {
    let pattern = r"^(q3_k|q3_k_l|q3_k_m|q3_k_s|q3_val|q3bits|q3bytes_0|q3bytes_1|q3bytes_2|q3bytes_3|q3h_0|q3h_1|q3h_2|q3h_3|q3l_0|q3l_1|q3l_2|q3l_3)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
