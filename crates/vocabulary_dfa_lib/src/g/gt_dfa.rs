use regex::Regex;

pub fn matches_gt(text: &str) -> bool {
    let pattern = r"^(gt1|gt2|gt_fi|gt_if|gt_neg_mid|gt_s|gt_s_token|gt_sreturn|gt_token|gtc|gtcc|gtci|gtcir|gtd|gtdo|gtdot|gte_fi|gte_if|gte_primitive|gtebaseenv15|gtebaseenv15q|gtelargeenv15|gtelargeenv15q|gteq|gterjsdcv8ky3hdz1ghbmodujypstuhyubx1vojh|gterm_blank_node|gterm_literal_language|gterm_literal_simple|gterm_literal_typed|gterm_named_node|gterm_triple|gterm_variable|gtermsource|gtk|gtk_edge_constraints|gtk_frame_extents_supported|gtk_im_module|gtksettings|gtl|gtlp|gtlpa|gtlpar|gtpoint|gtq|gtqu|gtque|gtques|gtquest|gtr|gtra|gtrap|gtrapp|gtrappr|gtrappro|gtrapprox|gtrar|gtrarr|gtrd|gtrdo|gtrdot|gtre|gtreq|gtreql|gtreqle|gtreqles|gtreqless|gtreqq|gtreqql|gtreqqle|gtreqqles|gtreqqless|gtriplesource|gtrl|gtrle|gtrles|gtrless|gtrs|gtrsi|gtrsim|gtumcz8ltnxvfxdrw7zsdftxxb7tutykzjnfwinpe6dg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
