use regex::Regex;

pub fn matches_rt(text: &str) -> bool {
    let pattern = r"^(rt2|rt_|rtable|rtavenar|rtb|rtc|rtc_track|rtcaudiosource|rtcvideosource|rtf_data|rtfd_data|rtfdfromrange|rtfdfromrange_documentattributes_|rtffromrange|rtffromrange_documentattributes_|rth|rthr|rthre|rti|rtim|rtime|rtimes|rtld_confgen|rtld_di_configaddr|rtld_di_linkmap|rtld_di_lmid|rtld_di_origin|rtld_di_profilename|rtld_di_profileout|rtld_di_serinfo|rtld_di_serinfosize|rtld_di_tls_data|rtld_di_tls_modid|rtld_first|rtld_group|rtld_next|rtld_nodelete|rtld_noload|rtld_parent|rtld_probe|rtld_self|rtld_world|rtlgetversion|rtline|rtm_target_feature|rtoalgorithm|rtok|rtoken|rtol|rtomax|rtomin|rtree|rtri|rtrie|rtrif|rtril|rtrilt|rtriltr|rtriltri|rtry_opt|rtryrecverror|rtrysenderror|rtspan|rtx|rtx_sign|rtxn|rty_mutbl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
