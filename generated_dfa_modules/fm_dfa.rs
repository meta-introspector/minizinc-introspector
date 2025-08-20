use regex::Regex;

pub fn matches_fm(text: &str) -> bool {
    let pattern = r"^(fm_arg|fm_args|fm_method|fm_method_name|fm_receiver|fm_span|fma4|fmaaa|fmaf32|fmaf64|fmcg|fmodules|fmp|fmpos|fms|fmt1|fmt2|fmt_args|fmt_cfgs|fmt_conf|fmt_const|fmt_dbg|fmt_debug|fmt_dep|fmt_desc|fmt_directory|fmt_dt|fmt_fields|fmt_for_test|fmt_helpers_for_derive|fmt_human_readable|fmt_index|fmt_input|fmt_int|fmt_interface_test|fmt_item|fmt_kw_as_variant|fmt_macro_argument|fmt_msg|fmt_opts|fmt_output|fmt_request_fields|fmt_shell|fmt_size|fmt_snippet|fmt_stmts_and_call|fmt_str|fmt_syms|fmt_syntax|fmt_trait|fmt_unsafe_arg|fmt_with_size|fmt_with_size_and|fmt_write_str|fmtarg|fmtconfig_path|fmtconfp|fmtcontext|fmtfloatconfig|fmtfulltypeguard|fmtid|fmtpadding|fmts|fmtsize|fmttypealias|fmtwriteinterface|fmtwritetrait|fmtˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
