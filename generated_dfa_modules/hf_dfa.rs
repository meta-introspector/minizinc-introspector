use regex::Regex;

pub fn matches_hf(text: &str) -> bool {
    let pattern = r"^(hf_attn_bias|hf_attn_weight|hf_datasets|hf_endpoint|hf_gpt2_model_id|hf_home|hf_hub_token|hf_import_statement|hf_llava_config|hf_preprocessor_config|hf_repo|hf_token|hf_transformer_prefix|hf_v5|hf_weight|hfaugmentedtermentry|hfaugmentedterms|hfgenerationconfig|hfinference|hfllavaconfig|hfllavatextconfig|hfllavavisionconfig|hfpdddnqjvcxnxkec697hddsyk6tfows2o8fkxuhqzpl|hfr|hfs4hsqki3luoydyqd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
