use regex::Regex;

pub fn matches_cd(text: &str) -> bool {
    let pattern = r"^(cdb_works_after_trimmed|cdecl_dash_unwind|cdeclunwind|cdefg|cdefgh|cdetrio1|cdetrio10|cdetrio11|cdetrio12|cdetrio13|cdetrio14|cdetrio15|cdetrio2|cdetrio3|cdetrio4|cdetrio5|cdetrio6|cdetrio7|cdetrio8|cdetrio9|cdist|cdkc8pptetnupozefcy5ayeturedkztnpmgz58nqyahd|cdmatchingiterator|cdo|cdoc|cdp|cdr_bad_input|cdr_definition|cdr_exists|cdr_handler|cdr_is_null|cdr_is_null_definition|cdr_mut|cdr_normal_input_2_elements|cdr_normal_input_3_elements|cdr_single_element_list|cdr_symbol|cdr_too_many_args|cdriehuys|cduration|cdylib_and_rlib|cdylib_both_forms|cdylib_extra_link_args_should_not_apply_to_unit_tests|cdylib_final_outputs|cdylib_link_arg_transitive|cdylib_not_lifted|cdylib_targets|cdˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
