use regex::Regex;

pub fn matches_bh(text: &str) -> bool {
    let pattern = r"^(bh1|bh2|bh_i|bhattacharyya|bhattacharyya_distance|bhl|bhmk2u6lqm2vnyf|bhq_account|bhts|bhtsne|bhvlngiqqkez8rpxch2ugjecic88zzewowpruoxpp1as|bhwc|bhwk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
