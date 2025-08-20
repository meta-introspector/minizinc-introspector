use regex::Regex;

pub fn matches_rm(text: &str) -> bool {
    let pattern = r"^(rm7kkflmprmm9cpwht6msjuewpdxlgwh9qdzijzgkhbmfyh63avcchjdauimqzvip7cud|rm_err|rm_file|rm_rf_glob|rm_rf_package_glob_containing_hash|rm_rf_prefix_list|rm_root|rmag|rmap|rmap_backward|rmap_err_interrogation_backward|rmap_err_interrogation_forward|rmap_forward|rmap_trivial_backward|rmap_trivial_forward|rmask|rmatch|rmatching|rmcloned|rmcpclient|rmdir|rmem_max|rmeta_time|rmethod|rmk|rmlpenw4v6qneehdjjvxo9xt99okgnufzs4y4375amw|rmo|rmou|rmous|rmoust|rmousta|rmoustac|rmoustach|rmoustache|rmove|rms_1|rms_2|rms_att_weight|rms_f32|rms_ffn_weight|rms_final_weight|rms_norm_cpu|rms_norm_gpu|rms_norm_metal|rms_norm_slow|rms_norml|rms_norml_cpu|rms_norml_gpu|rms_norml_metal|rmse|rmse_type|rmsnorm_bf16|rmsnorm_eps|rmsnorm_f16|rmsnorm_f32|rmsnorms|rmt|rmt2|rmul_lhs|rmul_rhs|rmupfdmedo04zvz5fqjjkvs7oyauyev4y|rmutexguard)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
