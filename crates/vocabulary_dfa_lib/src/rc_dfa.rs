use regex::Regex;

pub fn matches_rc(text: &str) -> bool {
    let pattern = r"^(rc1|rc4|rc_box_addr|rc_box_size|rc_buffer_info|rc_clone_in_vec_init_info|rc_file|rc_mutex_info|rc_refcell|rc_refcell_increment|rc_result|rc_slice_provider|rc_stash|rc_term|rc_test1|rc_test2|rc_test3|rc_test4_neg|rc_test5|rc_test6|rc_test7|rc_test8|rc_test9|rc_val|rc_value|rca|rcache|rcar|rcaro|rcaron|rcbox|rcd|rcdoc|rcdom|rced|rcedi|rcedil|rcei|rceil|rcf_expr|rcf_name_ref|rcf_pat|rcfile|rcfiles|rcgen|rchildren|rcho|rcho_basis|rcho_test|rchunks_exact_mut|rclrust|rcmonitor|rcmut|rcnormalposition|rcow_from|rcow_from_slice|rcow_from_str|rcow_into|rcowcompatibleref|rcpc|rcpc2|rcpc3|rcpointer|rcpy|rcrefcellwrapper|rcrefsteelval|rcs|rcslice|rcstr|rcstrstash|rcu|rcub|rcur|rcv|rcv_depth|rcv_path|rcv_ty|rcvbuf_errors|rcvbuf_errors_delta|rcvbuferrors|rcveciter|rcx|rcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
