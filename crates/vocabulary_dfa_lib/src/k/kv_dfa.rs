use regex::Regex;

pub fn matches_kv(text: &str) -> bool {
    let pattern = r"^(kv_a_layernorm|kv_a_proj_with_mqa|kv_b_proj|kv_cache_enabled|kv_cache_k|kv_cache_v|kv_channels|kv_chunks|kv_filters|kv_get|kv_head|kv_heads_cross|kv_input|kv_len|kv_lora_rank|kv_mapper|kv_mapper_lin|kv_memory|kv_n_heads|kv_out_dim|kv_repeat|kv_seq|kv_seq_len|kv_set|kv_split|kv_state|kv_states|kv_store_resource|kv_stream|kv_stride|kv_sz|kv_write|kva|kvals|kvar|kvarhelst|kven|kvi|kvifor|kvndeserializererr|kvp_key|kvpair)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
