use regex::Regex;

pub fn matches_o_(text: &str) -> bool {
    let pattern = r"^(o_|o_acc|o_append|o_as_ne|o_batch_stride|o_blockhash_entries|o_cargo|o_cargo_script|o_cfg_overrides|o_consts|o_cuti|o_discr|o_discr_ptr|o_discrs|o_enum|o_exhaus|o_extra_checks|o_f1|o_f2|o_fcount|o_field|o_field_abi|o_field_count|o_field_layout|o_fields|o_file|o_flag|o_func|o_gens|o_head_stride|o_index|o_key_list|o_kind|o_l1|o_l2|o_lay|o_len|o_list|o_map_prefix|o_match|o_matcher|o_name|o_names|o_nofollow|o_nonblock|o_package|o_prefix|o_prim|o_profile|o_project|o_ptr|o_r1|o_r2|o_rank|o_rdwr|o_row_stride|o_rustc|o_rustc_cfg|o_slice|o_str|o_stride|o_sysroot|o_t1|o_t2|o_tag|o_target_layout|o_term|o_toolchain|o_utid|o_ver_str|o_x)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
