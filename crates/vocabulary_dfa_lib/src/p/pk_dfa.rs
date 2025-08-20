use regex::Regex;

pub fn matches_pk(text: &str) -> bool {
    let pattern = r"^(pk1|pk2|pk3|pk4|pk_field|pk_with_1_ref|pk_with_2_refs|pkc|pkce_challenge|pkce_verifier|pkceclient|pkcecodechallenge|pkcehttpservice|pkceoauth2client|pkcequeryparams|pkceregistration|pkcs11|pkcs11_lib_path|pkcs1error|pkcs1key|pkcs1v15encrypt|pkcs8_prefix|pkcs8error|pkcs9|pkcs_ecdsa_p256_sha256|pkcuxadbc7qtbw8gikhl7agcsor|pkey_title|pkf|pkg0|pkg1|pkg2|pkg3|pkg_by_id|pkg_config_path|pkg_core|pkg_crates|pkg_data|pkg_default|pkg_dep_graph|pkg_dep_link|pkg_dep_with|pkg_dependencies|pkg_desc|pkg_df_false|pkg_df_true|pkg_feat_similar|pkg_feature_var|pkg_feature_var_map|pkg_features|pkg_hint_mostly_unused|pkg_id_loc|pkg_id_source|pkg_id_str|pkg_id_value|pkg_ids|pkg_lib_target|pkg_lints|pkg_loc|pkg_map|pkg_maybe_yanked|pkg_node_index|pkg_path|pkg_source_replacement_sid|pkg_std|pkg_to_lib_crate|pkg_var|pkgdir|pkgid_dir_containing_cargo_toml|pkgid_dir_plus_file|pkgid_dir_plus_path|pkgid_dir_to_nonexistent_cargo_toml|pkgid_json_message_metadata_consistency|pkgid_list_availables|pkgid_querystring_works|pkgids|pkgidspec|pkgname|pkgs_to_install|pkgs_to_prune|pkgs_to_scrape|pkh|pki_res|pkix|pkl|pks|pksc|pkt_batch_receiver|pkt_batch_sender|pkt_count|pkt_count_total|pkt_payload|pkt_receiver|pkvalue)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
