use regex::Regex;

pub fn matches_q8(text: &str) -> bool {
    let pattern = r"^(q81|q8_0_block_size|q8_2|q8_3|q8bytes|q8bytes_1|q8bytes_2|q8h|q8l|q8s|q8s_0|q8s_1|q8s_2|q8s_3|q8sums)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
