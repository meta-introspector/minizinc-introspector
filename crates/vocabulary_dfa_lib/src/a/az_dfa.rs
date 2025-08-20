use regex::Regex;

pub fn matches_az(text: &str) -> bool {
    let pattern = r"^(az2mb0bpw6i|az7eo4wzs7bmbogieyzppa0sjnedemtl9uqi7763xorniv|azb|azcd|azcz|azerbaijani|azeri|azerty|azhq8bia1grvvbcgyci7wzueswkgvu7yzvz4b9rkl5p6|azimuth|azk4|azok|azon|azonban|azp|azt|aztán|azure_api_key|azure_api_version|azure_default_api_version|azure_default_model|azure_doc_url|azure_endpoint|azure_ipv4_dynamic|azure_ipv4_virtual|azure_openai_deployment_name|azure_openai_endpoint|azure_openai_known_models|azure_provider_toml|azure_stack|azure_tests|azure_token|azureconfig|azurecredentials|azurefabricaddress|azuremessagecontent|azureopenaiauth|azureopenaichatchoice|azureopenaichatmessage|azureopenaichatmsg|azureopenaichatrequest|azureopenaichatresponse|azureopenaiembeddingdata|azureopenaifunctioncall|azureopenaitoolcall|azután|azvv9zzdxtgw4wwfjmsg6ytahpqgse1yz76nyy84vbqf|azy|azzal|azért)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
