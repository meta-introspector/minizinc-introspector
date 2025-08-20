use regex::Regex;

pub fn matches_f1(text: &str) -> bool {
    let pattern = r"^(f128_one|f128range|f16_100k|f16_10k|f16_1k|f16_ckernels|f16_count|f16_map|f16_one|f16_skernels|f16_vec|f16range|f19wo1rmw0x|f1_len|f1r2|f1r3|f1x|f1xr1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
