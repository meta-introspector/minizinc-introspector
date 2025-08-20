use regex::Regex;

pub fn matches_cc(text: &str) -> bool {
    let pattern = r"^(cc_config|cc_recognizer|ccall|ccap|ccaps|ccar|ccaro|ccaron|ccbin|cccdddˇ|ccedi|ccedil|cci|ccir|ccirc|ccittfaxdecode|ccmsenonsecurecall|ccmsenonsecureentry|cco|ccon|cconf|cconi|cconin|cconint|cconst|ccount|ccp|ccq|ccr|ccreate|ccryogenize|cctv|ccu|ccu4bommfluqcmftlphqiuo22zdusxjgzpauryawt1bw|ccup|ccups|ccupss|ccupssm|ccwtkflcpowjamxxrbkbe4cxyptlkgarzf8w1|cczyslxb2xjdgydvckvbm2owixqf7jf54ifodulj4g|ccˇc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
