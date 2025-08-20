use regex::Regex;

pub fn matches_bb(text: &str) -> bool {
    let pattern = r"^(bb8_postgres|bb8_redis|bb_items|bbar|bbaz|bbbbˇ|bbbcccˇ|bbbˇ|bbdata|bbox_attrs|bbox_buffer|bbox_struct|bbr|bbrk|bbrkt|bbrktb|bbrktbr|bbrktbrk|bbˇ|bbˇb_ccc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
