use regex::Regex;

pub fn matches_nm(text: &str) -> bool {
    let pattern = r"^(nmacro_rules|nmactiveconnection|nmactiveconnectionproxyblocking|nmake|nmarket|nmary|nmax|nmaximum|nmcp|nmdhcp4config|nmdhcp4configproxyblocking|nmeasuring|nmel|nmembers|nmessages|nmetadata|nmid|nmiie|nminimum|nmismatch|nmittler|nmmmm|nmn|nmnop|nmod_path|nmodule|nmore_flat|nmozilla|nms_basic|nms_threshold|nmt|nmtoken|nmtokens|nmulti|nmultiline|nmust|nmäry)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
