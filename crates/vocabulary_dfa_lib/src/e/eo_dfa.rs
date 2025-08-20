use regex::Regex;

pub fn matches_eo(text: &str) -> bool {
    let pattern = r"^(eo|eo1idtrzziakqfa8u431hedchasunpbu8mwg849mfvez|eod|eof_items|eof_line|eof_marker|eof_object|eof_object_definition|eof_objectp|eof_objectp_definition|eof_seen|eog|eogo|eogon|eoi|eol|eop|eopf|eopnotsupp|eos_ids|eos_position|eos_tok_id|eos_token2|eos_tokens|eos_vec|eostokenidvisitor|eot_id|eottoken)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
