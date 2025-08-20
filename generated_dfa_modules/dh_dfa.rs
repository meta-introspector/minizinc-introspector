use regex::Regex;

pub fn matches_dh(text: &str) -> bool {
    let pattern = r"^(dha|dhar|dharl|dharr|dhcp4|dhcp4_config|dhcp4config|dhcp6|dhcp_path|dhcp_server_identifier|dhole|dhsinglepass|dhsyfrjxfnh2g7hkjyszt79r74afa1wbhkaghndra1oy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
