use regex::Regex;

pub fn matches_mt(text: &str) -> bool {
    let pattern = r"^(mt5base|mt5large|mt5small|mt_0p5|mt_3|mt_8|mt_neg_1|mt_neg_2|mt_neg_2p5|mtable|mtch|mtch_arms|mtd|mteb|mtedlyi0nxx|mtext|mtfkfryd86jghlhfzvidcdo5cfrbmniyjijj8|mtg_12345|mtime_cache|mtime_duration|mtime_nanos|mtime_seconds|mtimenanos|mtimeseconds|mtl_capture_enabled|mtl_lasso_zero_works|mtlblendfactor|mtlblendoperation|mtlbuffer|mtlcapturedestination|mtlclearcolor|mtlcommandbuffer|mtlcommandbufferstatus|mtlcommandbufferstatuscommitted|mtlcomputecommandencoder|mtldatatype|mtldrawprimitivesindirectarguments|mtlloadaction|mtlmanaged|mtlprimitivetype|mtlprivate|mtlregion|mtlresourceusage|mtlsize|mtlstoragemode|mtlstoreaction|mtltexture|mtltexturetype|mtltextureusage|mtlviewport|mtmodel|mtnm0y0ewsycxuetmw1qspkgf8sqozkp6wdzyhlxu0i32gldxj4p24em|mtok|mtr|mts|mtsx|mtu_discovery_config|mtune|mtwo|mtwt|mty|mtype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
