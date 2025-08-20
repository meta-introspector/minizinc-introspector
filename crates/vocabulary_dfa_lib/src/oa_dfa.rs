use regex::Regex;

pub fn matches_oa(text: &str) -> bool {
    let pattern = r"^(oa|oac|oacu|oacut|oacute|oaep|oaep_sha256_padding|oaichatchoice|oaichatmessage|oaichatresponse|oaicite|oaicontentfilterresult|oaipromptfilterresult|oaistreamchoice|oaistreamchunk|oaistreamcollector|oaistreamdelta|oaitoolcall|oaitoolcallfunction|oaiusage|oak|oanb|oanbfoo_and_barˇ|oanbˇ|oast|oauth2config|oauth2securityscheme|oauth_|oauth_access_token|oauth_authorization_server|oauth_callback|oauth_client|oauth_client_id|oauth_client_secret|oauth_consumer_key|oauth_credentials|oauth_data|oauth_definition_store|oauth_domain|oauth_enabled|oauth_expires_in|oauth_handler|oauth_host|oauth_metadata|oauth_mutex|oauth_nonce|oauth_params|oauth_payload|oauth_pkce|oauth_refresh_token|oauth_request_payload|oauth_response|oauth_signature|oauth_signature_method|oauth_timestamp|oauth_token_type|oauth_url|oauth_verifier|oauth_version|oauthapiconfig|oauthcompute|oauthcustomerror|oauthdata|oauthflows|oauthlegacy|oauthlegacyhashalgorithm|oauthlegacysecret|oauthmissingcode|oauthpayload|oauthrequest|oauthstatemismatch|oauthtimeout|oauthtoken|oavh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
