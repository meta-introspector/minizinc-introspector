use regex::Regex;

pub fn matches_lh(text: &str) -> bool {
    let pattern = r"^(lh5dbadmhdhuozqc9tdwtk2mfituaxoc5zw3nz2oevvypqv5hn|lha|lhack|lhar|lhard|lharu|lharul|lhasa|lhb|lhbl|lhblk|lhd6rrvwscoqaz8xsbhiizi7jsxznksu|lhe|lhes|lhg6eph3wlz8yl|lhs0|lhs1|lhs3|lhs4|lhs_a|lhs_and_rhs_are_equal|lhs_bounds|lhs_buffer|lhs_chars|lhs_checker|lhs_child|lhs_children|lhs_const|lhs_cs|lhs_curr|lhs_device|lhs_dim1|lhs_dim2|lhs_dims|lhs_diverges|lhs_ele|lhs_entries|lhs_expectation|lhs_first|lhs_grad|lhs_guard|lhs_i|lhs_is_auto|lhs_is_different|lhs_is_let|lhs_is_simple_path|lhs_k|lhs_key|lhs_keys|lhs_kind|lhs_layout|lhs_len|lhs_lhs|lhs_m1|lhs_m2|lhs_name|lhs_ndims|lhs_operand|lhs_p|lhs_paren|lhs_permutation|lhs_prefix|lhs_proj|lhs_ref_counter|lhs_rhs|lhs_row|lhs_rs|lhs_s|lhs_snip|lhs_snippet|lhs_storage|lhs_str|lhs_stride|lhs_sugg|lhs_sum_grad|lhs_t|lhs_to_query_string|lhs_tree|lhs_ty_string|lhs_ty_string_elem|lhs_val|lhs_value|lhs_view|lhshelper|lhskind|lhslhs|lhsrhs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
