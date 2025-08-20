use regex::Regex;

pub fn matches_mk(text: &str) -> bool {
    let pattern = r"^(mk_args_trait|mk_block|mk_child|mk_clauses_from_iter|mk_doc_literal|mk_false|mk_fn_sig|mk_foo|mk_layout|mk_lbutton|mk_leaf|mk_lt_hint|mk_mbutton|mk_nonce_transaction|mk_param_from_def|mk_pound|mk_predicate|mk_punct|mk_ranges|mk_rbutton|mk_route|mk_spans|mk_struct2|mk_struct_full_def|mk_struct_with_field_name|mk_struct_with_names|mk_sysroot|mk_target|mk_ty|mk_type_list_from_iter|mk_xbutton1|mk_xbutton2|mka|mkfifo|mkfile|mkitem|mkl_name|mkn|mkoptions|mkopts|mkshell|mkstates|mktemp|mkv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
