use regex::Regex;

pub fn matches_ws(text: &str) -> bool {
    let pattern = r"^(ws_1|ws_2|ws_after|ws_and_sysroot|ws_before|ws_between|ws_caption|ws_child|ws_col|ws_config|ws_contents|ws_data|ws_def_feat|ws_dir|ws_directory|ws_document|ws_err_unused|ws_ex_appwindow|ws_ex_layered|ws_ex_toolwindow|ws_ex_topmost|ws_fingerprint|ws_manifest_path|ws_maximizebox|ws_metrics|ws_minimizebox|ws_msrv|ws_name|ws_original_toml|ws_path|ws_popup|ws_prefix|ws_root_config|ws_root_path|ws_root_reexport|ws_rustc_err|ws_setting_type|ws_structure_change|ws_suffix|ws_sysmenu|ws_thickframe|ws_tokenizer|ws_url_no_token|ws_version|ws_visible|ws_warn_path|ws_warn_unused|ws_wrapper|wsa_data|wsa_err|wsa_flag_overlapped|wsaewouldblock|wsasocketw|wsastartup|wsbuilder|wsc|wsconst_d|wsconst_d_filter|wsconst_g|wscr|wserror|wsf|wsfccj|wsize|wsjk|wslview|wspace|wspaces_before|wstr|wstrings|wsum)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
