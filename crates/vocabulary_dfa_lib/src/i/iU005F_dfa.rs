use regex::Regex;

pub fn matches_i_(text: &str) -> bool {
    let pattern = r"^(i_|i_1|i_16|i_2|i_32|i_64|i_8|i_a|i_am_eq|i_am_eq_too|i_am_just_right|i_am_public|i_area|i_block|i_care_less|i_cluster|i_cs|i_d|i_dont_wanna_use_your_name|i_elem_parsed|i_expr|i_f|i_hashmap|i_highlighted|i_in_block|i_k|i_lang|i_last|i_layer|i_level|i_name|i_next|i_over_2|i_p|i_ptr|i_range|i_result|i_rev_powers|i_right_broadcast|i_sep_parsed|i_should_also_be_hidden|i_should_be_hidden|i_token|i_up|i_val|i_value|i_x|i_xmax|i_xmin|i_y|i_ymax|i_ymin|i_zero)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
