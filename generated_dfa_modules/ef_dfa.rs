use regex::Regex;

pub fn matches_ef(text: &str) -> bool {
    let pattern = r"^(ef_for_ty|ef_search|ef_trie_psef_ranks_count_lm|efaaeaab8acqd6gx92zaiaaecdp4b2xeebsix7mqeung|efault|efbsg01wsduwaibr4covrzlngjxnyvtfw|efdo|efdot|effect_blocks|effected|effecting|effective_at|effective_call_id|effective_char_h_px|effective_current_url|effective_entries|effective_epoch|effective_full_mask|effective_id|effective_idx|effective_limit|effective_proj_mask|effective_rate_limited|effective_resolution|effective_rule_count|effective_source_id|effective_stake|effective_timeout|effective_timestamp|effective_url|effective_vis|effective_width|effectiveappearance|effectiveness|effectiverulecount|effectivestakeatrewardedepoch|efficientbert|efficientvit_|efficientvit_attn|efficientvit_block|efficientvit_downsample|efficientvit_head|efficientvit_model|efficientvit_msra|efficientvit_no_final_layer|efficientvit_stage|efficientvit_stem|efficientvit_stemblock|effient|effnet|effnet_c|effnet_embd|effnet_mappers|effortlessly|efforts|efghi|efgˇ|efhyd3safzgt472tyqduc4dpd2xdefks5fwkowugvt4w|efiapi|efloat|efoo|efr|efraimidis|eft|efter|efti)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
