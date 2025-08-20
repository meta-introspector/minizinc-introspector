use regex::Regex;

pub fn matches_pc(text: &str) -> bool {
    let pattern = r"^(pca_dataset|pca_embedding|pca_reduce|pcap|pcap_file|pcapng|pcapngreader|pcapreader|pcc|pccs|pcf|pcf_font_path|pcf_font_postscript_name|pch|pchampin|pchi7waaaabjru5erkjggg|pcij|pclmulqdq|pcm16|pcm_in|pcm_len|pcm_out|pcms|pcss|pct_err_from_arr1|pct_err_from_ds|pct_idle|pctx|pcwalton|pcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
