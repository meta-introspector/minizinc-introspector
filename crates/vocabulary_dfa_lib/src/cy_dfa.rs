use regex::Regex;

pub fn matches_cy(text: &str) -> bool {
    let pattern = r"^(cy7x5o3wi2eqhtoclmus6jswyx1ninbfw7axrrkrcpi8|cyanea|cybersecurity|cycle2|cycle2plus3|cycle5|cycle5_toxic|cycle_a|cycle_almost_pathological|cycle_b|cycle_counter|cycle_field|cycle_fn|cycle_initial|cycle_inline_completion|cycle_len|cycle_long|cycle_message_role|cycle_message_roles|cycle_next|cycle_params|cycle_pathological|cycle_potential|cycle_prev|cycle_through_same_place_diagnostics|cyclecollector|cycledetector|cycleitema|cycleitemb|cyclemessagerole|cyclemode|cyclenextinlineassist|cyclepreviousinlineassist|cyclerecoveryaction|cyclesreceived|cyclessent|cyclic_deps_rejected|cyclic_dev|cyclic_dev_dep|cyclic_dev_dep_doc_test|cyclic_feature|cyclic_feature2|cyclic_good_error_message|cyclic_module_reexport|cyclical_dep_with_missing_feature|cyclical_dev_dep|cyclicdependencieserror|cyclicirimapping|cyclicrule|cyclomatic|cyclomatic_complexity_threshold|cyclone_|cyclone_node|cyclone_scalar|cyclone_vector|cygnus|cyl|cylc|cylct|cylcty|cyou|cypher|cypripedium|cyrillic_script_in_unicode|cython)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
