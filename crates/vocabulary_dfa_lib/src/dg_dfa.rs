use regex::Regex;

pub fn matches_dg(text: &str) -> bool {
    let pattern = r"^(dgecode_root|dgemm|dgemm_|dgemm_ffi|dggml_metal_ndebug|dggml_use_clblast|dggml_use_cublas|dggml_use_metal|dghpcybpcybhihrlc3q|dgr|dgram|dgusaexxqecktjewwe|dgvzda|dgxaubefamwf0hczloxsovqv8vbj3gtnmntwzhyiblafsvtwsnosoikpb5bcukklanjq5arlcedthxzjaxg2wkq5lgvsccdxnatwxactwn9bgcxhfd1k4ltsamxkqq2patl1kamk4jibobzutk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
