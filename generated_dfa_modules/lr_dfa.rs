use regex::Regex;

pub fn matches_lr(text: &str) -> bool {
    let pattern = r"^(lr0|lr1|lr2|lr3|lr_contexts|lr_contexts1|lr_contexts2|lr_defaultsize|lr_expr|lr_lambda|lr_shared|lrar|lrarr|lrc|lrco|lrcor|lrcorn|lrcorne|lrcorner|lresult|lrgrwrks|lrh|lrha|lrhar|lrhard|lrhs|lrl|lrm|lrn|lro|lrpcrt4|lrt|lrtestparam|lrtr|lrtri|lru_cache|lru_cap|lru_capacities|lru_count|lru_parse_query_capacity|lru_query_capacities|lru_query_capacities_config|lru_score|lru_score_a|lru_score_b|lru_width|lrucapacity|lrudb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
