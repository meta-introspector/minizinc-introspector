use regex::Regex;

pub fn matches_rl(text: &str) -> bool {
    let pattern = r"^(rla|rlang_item|rlar|rlarr|rlater|rlh|rlha|rlhar|rlhs|rlib_with_debug|rlim_cur|rlim_max|rlimit|rlimit_nofile|rlist|rll|rllama|rlm|rlmsrpol8p1t)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
