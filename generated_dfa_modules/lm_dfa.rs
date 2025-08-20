use regex::Regex;

pub fn matches_lm(text: &str) -> bool {
    let pattern = r"^(lm_forward|lm_forward_embeds|lm_head_tensor|lm_idx|lmal|lmaolmaolmao|lmap|lmethod|lmgetkbdtype|lmh_b|lmi|lmid|lmido|lmidot|lmk|lmo|lmou|lmous|lmoust|lmousta|lmoustac|lmoustach|lmoustache|lmoˇ|lmsimg32|lmstudio_api_url|lmstudio_catalog_url|lmstudio_download_url|lmstudio_intro|lmstudio_site|lmstudioembeddingmodel|lmstudioembeddingprovider|lmstudioembeddingrequest|lmstudioembeddingresponse|lmstudioeventmapper|lmstudiolanguagemodel|lmstudiosettingscontent|lmsvcrt|lmswsock|lmsys|lmul_lhs|lmul_rhs|lmy|lmys)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
