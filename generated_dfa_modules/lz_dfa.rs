use regex::Regex;

pub fn matches_lz(text: &str) -> bool {
    let pattern = r"^(lz|lz4_encoder|lz4_flex|lz4block|lzma|lznumx|lzrndkxzmazb5cjmwq0l9p1o5q|lzy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
