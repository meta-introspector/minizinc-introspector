use regex::Regex;

pub fn matches_fw(text: &str) -> bool {
    let pattern = r"^(fw|fw32|fw64|fw_normal|fwany|fwcd|fwd_pos|fwd_thread_hdl|fwd_token|fwdansi|fwfaykn7acnseudhanzghqtgqzmcgnussahahuqbdprz|fword|fwpvdpvumnco1csfwxqdtbubuhg5ep7h2vgckykvl7at)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
