use regex::Regex;

pub fn matches_d_(text: &str) -> bool {
    let pattern = r"^(d__e|d__gcc_have_dwarf2_cfi_asm|d_a|d_all|d_auditor|d_b|d_c|d_conv|d_crate_graph|d_crt_secure_no_warnings|d_data|d_dest|d_destination|d_dll|d_dx_sigmoid|d_elems|d_errs|d_ff|d_first|d_head|d_id|d_idx|d_inner|d_k|d_kv|d_latent|d_mbcs|d_mut|d_proc_macros|d_rounded|d_second|d_source|d_sqrt|d_state|d_str|d_third|d_tree|d_use_math_defines|d_w_j|d_w_max|d_w_tol|d_windows)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
