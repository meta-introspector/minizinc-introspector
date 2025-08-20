use regex::Regex;

pub fn matches_rs(text: &str) -> bool {
    let pattern = r"^(rs0|rs256error|rs256keypair|rs256signatureerror|rs_b|rs_buffer|rs_editor|rs_ext|rs_fake_servers|rs_file|rs_files_recursively|rs_lsp_request_count|rs_name|rs_type|rs_u32|rs_vs_rs|rsa_bits|rsa_key|rsa_pkcs1_sha1|rsa_pkcs1_sha256|rsa_pkcs1_sha384|rsa_pkcs1_sha512|rsa_pss_sha256|rsa_pss_sha384|rsa_pss_sha512|rsaencryption|rsaes|rsaprivatekey|rsaq|rsaqu|rsaquo|rsasignaturewithripemd160|rsassa|rsbox|rsc1|rsc2|rscr|rse|rseg|rsenderror|rsendtimeouterror|rsfile|rsh|rsi|rsib|rslt|rslt0|rslt1|rsm|rsnip|rsomething|rsomething_cto|rsomething_else|rsomething_methods|rsomething_to|rsomethingelse|rsomethingelse_to|rspace|rspan|rsplit_char|rsplitn_char|rsplitn_mut|rsq|rsqb|rsqu|rsquo|rsquor|rsrc|rss_delta|rss_delta_kb|rssanon|rssfeed|rssfeederror|rst_stream|rstar|rstar_network_create|rstart|rstmt_span|rstr_cap|rstr_interface|rstride|rstring_ctor|rstring_interface|rsvp|rsync|rsyntaxkind|rsˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
