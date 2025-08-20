use regex::Regex;

pub fn matches_fx(text: &str) -> bool {
    let pattern = r"^(fx_best|fx_hash_map|fx_hash_set|fx_prev|fxhasmap|fxs1zh47qbnnhxcnb6yiaqoj4sgb91tkf3ufhlckt7pm|fxx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
