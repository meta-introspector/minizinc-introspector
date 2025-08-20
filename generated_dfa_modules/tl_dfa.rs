use regex::Regex;

pub fn matches_tl(text: &str) -> bool {
    let pattern = r"^(tl_data|tl_enums|tl_field_accessor_macro|tl_field_macro|tl_fields|tl_function|tl_functions|tl_lifetimes|tl_lifetimes_macro|tl_multi_tl_macro|tl_offset|tl_other|tl_prefix|tl_primitive|tl_reflection|tl_sv_steelval|tl_syntax_object_id|tl_type_layout_index|tl_v_steelval|tlast|tlb|tld|tldiscrsinner|tldr|tleft_id|tlex_type|tlf_a|tlf_b|tlfieldshallow|tli12|tline_end|tline_start|tlp|tls12|tls13|tls13_aes_128_gcm_sha256|tls13_aes_256_gcm_sha384|tls_acceptor|tls_certificates|tls_cfg|tls_connector|tls_grpc_channel|tls_index|tls_rustls|tls_server_roots|tls_session|tls_tcp_reader|tls_tcp_writer|tlsdata|tlserror|tlskind|tlsoptions|tlssession|tlsv1|tlsv10|tlsv11|tlsv13)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
