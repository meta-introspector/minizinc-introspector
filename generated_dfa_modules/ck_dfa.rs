use regex::Regex;

pub fn matches_ck(text: &str) -> bool {
    let pattern = r"^(ck_attribute|ck_attribute_type|ck_key_type|ck_mechanism|ck_object_class|ck_object_handle|ck_session_handle|ck_slot_id|ck_void|ck_void_ptr|cka_class|cka_ec_params|cka_ec_point|cka_id|cka_key_type|ckdy0x5dnhuhqz0wfto4j|ckf_login_required|ckf_serial_session|ckk_ecdsa|ckm_ecdsa|cko_private_key|cko_public_key|ckr_cryptoki_already_initialized|cksum010|cksum020|cksum_file|cktruq2mttgrgkxjtyksdkhjudc2c4tgdzyb98oezy8|cku_user|ckv_split)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
