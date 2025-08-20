use regex::Regex;

pub fn matches_ur(text: &str) -> bool {
    let pattern = r"^(ur64|ura|urania|urc|urchade|urchin|urco|urcor|urcorn|urcorne|urcorner|urcr|urcro|urcrop|urdf|urdf_|urdf_path|urdfs|urdu|urecip|uref|urelu|urg|urge|uri_and_prefix|uri_base|uri_clone|uri_count|uri_for_buffer|uri_host|uri_list|uri_patterns|uri_prefix|uri_query|uri_str|uri_string|uri_to_emoji|uri_to_source_id|uri_without_scheme|uriandprefix|uriext|urimodifierinterceptor|urin|uriparse|uripattern|urireference|urireferencebuilder|urireferenceerror|uritemplate|url1|url2|url3|url4|url5|url_2|url_attempt|url_channel|url_content|url_display|url_encoded|url_field|url_finder|url_for|url_for_authentication|url_label|url_latest|url_link_color|url_match|url_matches|url_norm_hash|url_not_supported|url_nsstring|url_offsets|url_or_path|url_override|url_parsed|url_patterns|url_prefixes|url_protocols|url_provided_is_invalid|url_query|url_range|url_recognizer|url_regex|url_rewrite|url_setting_type|url_strings|url_text_color|url_to_code_description|url_to_fetch|url_to_open|url_with_base|url_with_token|url_workflow|urldecoded|urlencoder|urlforapplicationwithbundleidentifier|urlforauxiliaryexecutable|urllib3|urls_iter|urls_to_open|urlsearchparams|urna|urocyon|uround|urru|ursinus|ursus|urt|urtr|urtri|uruguay)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
