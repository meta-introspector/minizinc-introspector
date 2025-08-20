use regex::Regex;

pub fn matches_ir(text: &str) -> bool {
    let pattern = r"^(ir_generation|ir_module|ir_version|irdfoo|iref_enum|ireg_name|ireland|irelative|irelative_part|irelative_ref_regex|irelative_ref_regex_src|iretry|irg0apz217ah2rxxjpquwysxueuap8ylbsjqw8exgqvj|iri1|iri2|iri_as_term|iri_box|iri_from_scratch|iri_from_term|iri_from_term_ref|iri_index|iri_mapping|iri_not_supported|iri_or_path|iri_ref_box|iri_ref_regex|iri_regex|iri_regex_src|iri_string|iri_to_native|iri_triple|iri_with|iriarg|iriattribute|irienum|iriexpansionresult|irimustendwithslash|irinotabsolute|iriorindex|iriorpath|iriparseerror|iriref_as_term|iris_of|iris_separate|irishextended|irm|ironic|ironman|irq|irrecoverable|irreflexiveproperty|irrefutable_match|irrefutable_slices|irregular|irrelevant_bounds_are_filtered_out|irrep_test|irrep_test_internal|irritating|irroratus|irsplicing|irst|irty_five|irure)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
