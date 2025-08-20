use regex::Regex;

pub fn matches_ix(text: &str) -> bool {
    let pattern = r"^(ix0|ix_attempted_allocation_size|ix_bytes|ix_c|ix_converter|ix_cost|ix_count|ix_discriminator|ix_e550_r256|ix_e600_r384|ix_entry|ix_entry_raw|ix_error_signature|ix_error_tx|ix_iter|ix_len|ix_new|ix_offset|ix_old|ix_program_id|ix_within_line|ixi|ixn_string|ixns|ixvbc2ymauu8hiofupxtyqfnzg5fq0rhjsewdtywxiadjslj6fsk|ixx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
