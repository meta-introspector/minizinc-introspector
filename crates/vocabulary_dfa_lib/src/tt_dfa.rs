use regex::Regex;

pub fn matches_tt(text: &str) -> bool {
    let pattern = r"^(tt3qvmbi|tt_conversion|tt_delimiter|tt_level|tt_result|tt_stack|ttfb|ttft|ttft_duration|ttft_time|tthat|tthe|tthey|ttitersavepoint|ttl_content|ttl_mappings|ttl_secs|ttlmapping|ttm|ttmsubtxdxwscnjhhk54fs72yudfbds6s|tto|ttotal_cost|ttree_to_expr_list|tts_builder|tttreesink|ttydxl_master_left|ttydxl_master_right|ttydxl_puppet_left|ttydxl_puppet_right|ttyusb0|ttywidth)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
