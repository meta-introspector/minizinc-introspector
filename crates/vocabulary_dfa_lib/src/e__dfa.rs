use regex::Regex;

pub fn matches_e_(text: &str) -> bool {
    let pattern = r"^(e____t|e_adt|e_binop|e_blinding|e_clone|e_cocci_|e_conn|e_conn_lock|e_data|e_dim|e_expand|e_expn|e_fail|e_fb|e_idx|e_invalidarg|e_lhs|e_ly|e_meta|e_notimpl|e_ptr|e_ref|e_rhs|e_seg|e_shape|e_shop|e_step|e_string|e_tag|e_txt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
