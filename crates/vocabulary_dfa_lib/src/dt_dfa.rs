use regex::Regex;

pub fn matches_dt(text: &str) -> bool {
    let pattern = r"^(dt1|dt2|dt2tuple|dt4|dt4n6abdqs6w4bnfwrxt9rsprcpf6cddga1egctapklc|dt_from_ts_nanos|dt_proj|dt_rank|dt_str|dt_term|dt_utc|dt_with_fixed_tz|dtdo|dtdot|dtds|dtermsource|dti8aijyrneroefk7i7gjal8dldmm|dtoa|dtr|dtri|dtrif|dts|dtvtkmw3jsofd8cjvjte8pxebxnq2yzijvvr3pe2appj|dtw|dtw_timestamps|dtype_fn|dtype_str|dtypefunction)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
