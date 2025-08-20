use regex::Regex;

pub fn matches_vk(text: &str) -> bool {
    let pattern = r"^(vk_|vk_0|vk_1|vk_2|vk_3|vk_4|vk_5|vk_6|vk_7|vk_8|vk_9|vk_abnt_c1|vk_apps|vk_back|vk_browser_back|vk_browser_forward|vk_capital|vk_control|vk_delete|vk_down|vk_end|vk_escape|vk_f1|vk_f10|vk_f11|vk_f12|vk_f13|vk_f14|vk_f15|vk_f16|vk_f17|vk_f18|vk_f19|vk_f2|vk_f20|vk_f21|vk_f22|vk_f23|vk_f24|vk_f3|vk_f4|vk_f5|vk_f6|vk_f7|vk_f8|vk_f9|vk_home|vk_insert|vk_left|vk_lwin|vk_menu|vk_next|vk_oem_1|vk_oem_102|vk_oem_2|vk_oem_3|vk_oem_4|vk_oem_5|vk_oem_6|vk_oem_7|vk_oem_8|vk_oem_comma|vk_oem_minus|vk_oem_period|vk_oem_plus|vk_packet|vk_prior|vk_processkey|vk_return|vk_right|vk_rwin|vk_shift|vk_space|vk_tab|vk_up|vkdiscr|vkern|vkey|vks)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
