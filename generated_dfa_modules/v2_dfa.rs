use regex::Regex;

pub fn matches_v2(text: &str) -> bool {
    let pattern = r"^(v2023_11_09|v2025_03_26|v22|v23|v2_0_0|v2_1_|v2_70b|v2_7b|v2_base|v2_bins|v2_bytes|v2_chap02|v2_data|v2_features|v2_format_preserved|v2_index|v2_info|v2_lock|v2_lockfile|v2_path|v2_path_and_crates_io|v2_ref|v2_syncs|v2_table|v2basepatch16_224|v2basepatch16_256|v2basepatch16_384|v2basepatch16_512|v2chat|v2f_map|v2inpaint|v2j2pqglcpvlihcw0b9nh8|v2largepatch16_256|v2largepatch16_384|v2largepatch16_512|v2old|v2r_map|v2r_maps|v2w1b|v2zephyr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
