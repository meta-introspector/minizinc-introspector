use regex::Regex;

pub fn matches_lt(text: &str) -> bool {
    let pattern = r"^(lt_a|lt_attr|lt_bound|lt_de_erasedptr|lt_erasedptr|lt_fi|lt_generic_comp|lt_hash1|lt_hash2|lt_hash_bytes|lt_hash_time|lt_hashes|lt_hints|lt_i|lt_ident|lt_if|lt_mid|lt_num|lt_opt|lt_rbox|lt_ref|lt_return|lt_return_only|lt_rmut|lt_rref|lt_sub_lt|lt_token|lt_tokens|ltag1|ltag2|ltarget|ltbl|ltc|ltcc|ltci|ltcir|ltda|ltdlt9ycbyoipz5flysci1nndnasszfmjljxts5zxzz|ltdo|ltdot|lte2|lte_binop|lte_fi|lte_handler|lte_handler_none_int|lte_handler_payload|lte_if|lte_primitive|lteimmediate|lteimmediateif|lteq|lteregister|ltgetterelider|lth|lthashqx6661dadd4s6a2tfi6qbuiwxkv66fb1obfhq|lthe|lthr|lthre|lthree|lti|ltim|ltime|ltimes|ltko|ltl|ltla|ltlar|ltlarr|lto2|lto_args|lto_build|lto_when_needs_object|ltokens|ltopt|ltq|ltqu|ltque|ltques|ltquest|ltri|ltrie|ltrif|ltrp|ltrpa|ltrpar|lts_idx_iter|ltsnap8h1voevvtomnbnqoinqex4aqfurbfhrh3msq2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
