use regex::Regex;

pub fn matches_lc(text: &str) -> bool {
    let pattern = r"^(lc_ctype|lc_threshold|lca_index|lca_map|lca_node|lcar|lcaro|lcaron|lcase|lcd|lce|lced|lcedi|lcedil|lcei|lceil|lcfgmgr32|lcp|lcpy|lcredui|lcrypt32|lcryptnet|lcs|lcu|lcub|lcur|lcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
