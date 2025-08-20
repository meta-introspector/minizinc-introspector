use regex::Regex;

pub fn matches_t5(text: &str) -> bool {
    let pattern = r"^(t5_3b|t5_emb|t5_embeddings|t5a|t5attention|t5b|t5base|t5block|t5denseactdense|t5densegatedactdense|t5large|t5layercrossattention|t5layerff|t5layernorm|t5layerselfattention|t5modelbuilder|t5small|t5stack|t5withtokenizer|t5xxl|t5xxl_file|t5xxl_fp16)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
