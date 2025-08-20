use regex::Regex;

pub fn matches_ae(text: &str) -> bool {
    let pattern = r"^(ae5vq|ae_ciphertext_base64_str|ae_ciphertext_fromstr|ae_ciphertext_max_base64_len|ae_key_from_path|ae_key_from_seed_phrase|ae_key_from_source|ae_key_str|aeads|aeghhgwpe|ael|aeli|aelig|aenean|aepyceros|aeroplane|aes128gcmsiv|aes256ctr|aes_key|aesnq|aexplicit_builtin_cfgs_in_flags)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
