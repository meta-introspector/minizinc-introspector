use regex::Regex;

pub fn matches_sg(text: &str) -> bool {
    let pattern = r"^(sg_config|sg_label_to_code_span_label|sg_node|sg_root|sgd_linear_regression|sgd_optim|sgemm|sgemm_|sgemm_ffi|sglangerr|sgml|sgnatures_out|sgnodematch|sgr|sgr_mouse|sgr_mouse_report|sgs_clock|sgs_epoch_rewards|sgs_epoch_schedule|sgs_rent|sgx|sgx_platform)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
