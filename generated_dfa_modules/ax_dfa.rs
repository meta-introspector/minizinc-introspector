use regex::Regex;

pub fn matches_ax(text: &str) -> bool {
    let pattern = r"^(axes_dim|axes_set|axes_settings|axf|axioms|axis0|axis1|axis2|axis3|axis_i64|axis_iter_mut|axis_label|axis_modifier|axis_name_func|axis_source|axis_text|axis_x|axis_y|axisbuilder|axisdiscrete|axisscale|axisside|axissource|axisvalue120|axisvalues|axnoderef|axolotl|axum1|axum2|axum_embed|axum_error|axum_macros_debug|axum_main|axum_prometheus|axum_server|axum_test_only|axumassets|axumcloseframe|axumformrejection|axummessage|axˇ|axˇdexˇg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
