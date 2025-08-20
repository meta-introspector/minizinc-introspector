use regex::Regex;

pub fn matches_et(text: &str) -> bool {
    let pattern = r"^(eta_duration|eta_id|eta_invariant|eta_reduction|eta_seconds|etcd_cacert_file|etcd_cert_file|etcd_client|etcd_domain_name|etcd_endpoint|etcd_key_file|etcdtlsconfig|etcdtowerstorage|etched|etdc_to_tower_error|ete|eternal_ai_agent_id|eternal_ai_contract|eternal_ai_rpc|eternalai_agent_contract_address|eternalai_agent_id|eternalai_api_base_url|eternalai_api_key|eternalai_rpc_url|eternity|eth|eth0|eth1|eth_address|eth_address_instruction_index|eth_address_offset|eth_address_slice|eth_config|ethereal|ethereumwallet|ethernet|ethernet_mac_address|ethers|ethical|ething|ethiopic|etimedout|etoolcallargument|ett|ette|etter|että|etude|eturn|etwas|etx|ety|etype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
