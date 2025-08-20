use regex::Regex;

pub fn matches_uu(text: &str) -> bool {
    let pattern = r"^(uua|uuar|uuarr|uuid1|uuid2|uuid3|uuid_buf|uuid_group_lens|uuid_ids|uuid_or_name|uuid_phase1_discovery|uuid_phase3_resolution|uuid_start|uuid_v4|uuid_val|uuid_val2|uuid_val3|uuid_wrapper|uuidconv|uuidgen|uuidv5|uuidversion|uum|uuml|uutils|uuus3k)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
