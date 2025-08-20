use regex::Regex;

pub fn matches_v1(text: &str) -> bool {
    let pattern = r"^(v11|v11_0|v128_and|v128_load|v128_or|v128_store|v12_0|v13|v16_0|v17|v17_0|v1_0|v1_0_0|v1_0_5|v1_0h|v1_0l|v1_17_14|v1_1_0|v1_1_incompatible|v1_2|v1_3|v1_3_incompatible|v1_5_0|v1_5inpaint|v1_7_14|v1_7w7b|v1_already_installed_dirty|v1_already_installed_fresh|v1_bins|v1_data|v1_environment|v1_lock|v1_lockfile|v1_path|v1_ref|v1_table|v1basepatch16_224|v1mmdz8m7t2f8uhqk|v1orig|v1p9|v1preview|v1zephyr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
