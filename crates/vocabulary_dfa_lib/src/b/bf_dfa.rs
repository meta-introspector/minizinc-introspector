use regex::Regex;

pub fn matches_bf(text: &str) -> bool {
    let pattern = r"^(bf16_100k|bf16_10k|bf16_1k|bf16_ckernels|bf16_map|bf16_skernels|bf16_vec|bf_repo|bfeat|bfgs|bfloat16storage|bfoo|bford|bfr|bfs_detector|bfs_paths|bft)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
