use regex::Regex;

pub fn matches_qn(text: &str) -> bool {
    let pattern = r"^(qnt_version|qnt_version_factor|qnx|qnxt7ctpmqu6unlpsnzq65xloqkxo71vq3c2me)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
