use regex::Regex;

pub fn matches_ak(text: &str) -> bool {
    let pattern = r"^(akamai_instance_host_uuid|akamai_instance_id|akamai_instance_label|akamai_instance_region|akamai_instance_tags|akamai_instance_type|akamai_ipv6_link_local|akamai_ipv6_range_|akamai_ipv6_shared_range_|akamai_ipv6_slaac|akamai_private_ipv4_|akamai_private_ipv4_0|akamai_public_ipv4_|akamai_public_ipv4_0|akamai_shared_ipv4_|akash|akd3s|akey|aki|akiaiosfodnn7example|akiatest|akid|akihgcppudrvy2civrpsdxeea7bz5aquime0eold56n3o7e5bjuaozf|akik|akind|akj7yssrqs3x4uwlusptxbp6lfvgdpybwh4jgk5eetgz|akkor|akwirw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
