use regex::Regex;

pub fn matches_dc(text: &str) -> bool {
    let pattern = r"^(dc2ohxfxqac2qflstuu7txtd3u5hz82mrcsgdoowjbsv|dc5iree6xclod|dc_target|dc_test|dc_variant|dcar|dcaro|dcaron|dchild|dcmake_build_type|dcmake_c_flags|dcmake_cxx_flags|dcmake_policy_version_minimum|dcou|dctdecode|dcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
