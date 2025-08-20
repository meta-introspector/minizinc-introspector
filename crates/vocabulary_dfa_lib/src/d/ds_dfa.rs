use regex::Regex;

pub fn matches_ds(text: &str) -> bool {
    let pattern = r"^(ds1|ds87kveqhbv7jw8w6avss1mqz3mw5j3prtppodq2qdij|ds_orig|ds_proj|ds_s|ds_s_score|ds_score|dsbt|dsc|dscr|dsculley|dscy|dserializing|dsfs|dsharp|dsknyr8cpucrbx2vyssz7yx3iirqngd38vqvahkuvgv1|dso|dsol|dsomething|dsomething_methods|dsomething_to|dsp|dst0|dst1|dst_access|dst_addr|dst_base_str|dst_big|dst_big_f16|dst_buf|dst_buffer|dst_c_idx|dst_cfg|dst_chunk|dst_chunk_addr|dst_chunk_iter|dst_cs|dst_dim|dst_dim_len|dst_dim_size|dst_dim_sz|dst_el|dst_f16|dst_h|dst_host_addr|dst_id|dst_in_path|dst_kind|dst_lamports|dst_left_len|dst_limit|dst_metadata|dst_name|dst_o|dst_o_in_bytes|dst_opts|dst_point|dst_principal|dst_rect|dst_region|dst_remaining|dst_right_len|dst_row|dst_rs|dst_s|dst_s0|dst_s1|dst_s2|dst_s3|dst_slice|dst_slot_output|dst_storage|dst_stride1|dst_strides|dst_sz|dst_to_set|dst_tty|dst_uv|dst_uv_buffer|dst_uv_len|dst_uv_stride|dst_v|dst_va|dst_vec|dst_vm_addr|dst_w|dst_x|dst_y_buffer|dst_y_len|dst_y_stride|dstart|dstro|dstrok|dstyal1sqqlrpnlg|dsu|dsymutil)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
