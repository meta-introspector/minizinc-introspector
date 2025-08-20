use regex::Regex;

pub fn matches_ps(text: &str) -> bool {
    let pattern = r"^(ps2|ps_arg|ps_check|ps_id|ps_path|psc|pscaled|pscr|psegment|psess_created|pseudo_bfloat16_config|pseudo_code_skeleton|pseudocolon|pseudoheader|pseudorandom|pseudoroot|pseudoscalar_part|psh|psibs|psiitemarray|psittacus|psm1|psnode|psp|psqr|pss|pst|psuedo|pszbuttontext|pszcontent|pszmainicon|pszmaininstruction|pszname|pszspec|pszwindowtitle)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
