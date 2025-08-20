use regex::Regex;

pub fn matches_m_(text: &str) -> bool {
    let pattern = r"^(m_|m_1|m_2|m_2025_01_02|m_2025_01_29|m_2025_01_30|m_2025_03_03|m_2025_03_06|m_2025_03_29|m_2025_04_15|m_2025_04_21|m_2025_04_23|m_2025_05_05|m_2025_05_08|m_2025_05_29|m_2025_06_16|m_2025_06_25|m_2025_06_27|m_2025_07_08|m_4|m_any_id|m_arm|m_arms|m_cache|m_cast|m_child|m_docs|m_dtype|m_expr|m_goalstate|m_hat|m_health|m_hostname|m_id|m_ident_span|m_identity|m_idx|m_idx_str|m_inner|m_k|m_path_string|m_pos|m_recv|m_size|m_start_angle|m_step|m_target|m_version|m_vmsize)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
