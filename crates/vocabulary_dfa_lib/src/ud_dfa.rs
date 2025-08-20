use regex::Regex;

pub fn matches_ud(text: &str) -> bool {
    let pattern = r"^(ud|uda|udar|udarr|udb|udbl|udbla|udblac|udev|udevadm|udf|udh|udha|udhar|udiv|udivide|udm|udm_database_name|udm_database_url|udm_db_name|udm_db_url|udn|udp_config|udp_lines|udp_port|udp_ports|udp_socket_with_config|udp_sockets|udp_stats|udp_timer|udp_tpu|udpbaseclientconnection|udpclientconnection|udpconfig|udpconnectionmanager|udplite|udppool|udpsocketpair|udpstats|uds)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
