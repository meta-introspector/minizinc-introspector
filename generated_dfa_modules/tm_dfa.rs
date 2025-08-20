use regex::Regex;

pub fn matches_tm(text: &str) -> bool {
    let pattern = r"^(tmap|tmatchinto|tmd|tme_leave|tme_nonclient|tmi|tminfo|tmodule|tmodule_path|tmodule_two_expected_id|tmp3|tmp4|tmp6a|tmp6b|tmp6c|tmp_breakpoint|tmp_buf|tmp_buffer|tmp_dir_str|tmp_extract_dir|tmp_file2|tmp_file_for_splitted_blocks|tmp_file_id|tmp_file_name_re|tmp_filename|tmp_files_for_splitted_blocks|tmp_genesis_package|tmp_genesis_path|tmp_init|tmp_ledger_name|tmp_len|tmp_native_owner|tmp_out|tmp_outfile_path|tmp_pack_9kusa8|tmp_path_gz|tmp_reg|tmp_res|tmp_schema|tmp_scores|tmp_snapshot_archive_prefix|tmp_table|tmp_ty|tmp_var|tmp_vtable|tmp_wm|tmp_word|tmp_ws|tmpdir1|tmpdisj|tmpdisjs|tmpfile_dir|tmpfile_path|tmpl_key|tmpm|tmpmod|tmpnode|tmpp|tmpregistry|tmpvec|tmpx|tmutex|tmvars)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
