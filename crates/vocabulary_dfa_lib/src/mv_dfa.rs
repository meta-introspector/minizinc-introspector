use regex::Regex;

pub fn matches_mv(text: &str) -> bool {
    let pattern = r"^(mv1|mv2|mv_0|mv_00|mv_01|mv_1|mv_10|mv_11|mv_a|mv_b|mv_c|mv_char|mv_d|mv_dense|mv_i|mv_j|mv_ret|mv_sparse|mva|mvar1|mvar2|mvc|mvccrwlock|mvn1|mvn2|mvomz2ag9at40ldnxxrhjl1by6qsqsfdq3bznc5gm|mvp|mvq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
