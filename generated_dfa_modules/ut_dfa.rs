use regex::Regex;

pub fn matches_ut(text: &str) -> bool {
    let pattern = r"^(ut8|ut_type|utah|utan|utanh|utc_datetime|utc_rfc3339|utctime|utd|utdo|utdot|uten|uterm|utf16_count|utf16_cu_to_char|utf16_end|utf16_extra|utf16_idx|utf16_ix|utf16_len|utf16_line_len|utf16_start|utf16_target|utf16selection|utf32buf|utf32str|utf8_additional_len|utf8_codepoint_width|utf8_count|utf8_decode|utf8_end|utf8_full|utf8_identifiers_test|utf8_idents|utf8_index|utf8_ix|utf8_mime_0|utf8_mime_1|utf8_mouse|utf8_or_array|utf8_path|utf8_path_buf|utf8_round_corners|utf8_sequence_0|utf8_symbols|utf8_target|utf8codeunitoffsetfromlinestart|utf8components|utf8frompath|utf8frompathbuf|utf8readerror|utf8type|utf8typedpath|utf8view|utf_8|uti|util_dir|util_dirs|util_guide|util_test_search_in_block|util_test_search_in_block_all|utilcmd|utild|utilde|utilises|utility_rule|utilityfunctions|utilityhelper|utilpat|utils_universal_openai_stream|utl4gdcni9vm8qmmgdrloi|utmost|utmp|uto_params|uto_params_use|utolsó|utr|utraau|utree_sitter_reuse_allocator|utri|utrif|utrr|utrtr|utrtrtr|utrtrtra|utru|utruaua|utrutru|utsname|utt|uttarru|utteranceid|utterances|uttu|uttype|uturu|utxo|utxooutpoint|utxos|utxosfilter|utype|után|utána)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
