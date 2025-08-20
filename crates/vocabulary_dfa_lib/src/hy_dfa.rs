use regex::Regex;

pub fn matches_hy(text: &str) -> bool {
    let pattern = r"^(hy|hyaena|hyb|hybrid_inference|hybrid_large|hybrid_medium|hybrid_reasoning|hybrid_results|hybridlarge|hybridmedium|hybridsearch|hybu|hybul|hybull|hydra|hydrant|hyena|hygiene_for_range|hygiene_id_for|hygiene_info|hylobates|hymn|hymn_tune|hyp|hype_content|hype_level|hypecycle|hyper_014|hyper_client|hyper_in|hyper_in_list|hyper_lazy|hyper_proxy|hyper_pump|hyper_pump_calculation|hyper_pump_mechanism|hyperbolic_api_base_url|hyperclientbuilder|hypererror|hyperfork|hypergracefulconnection|hyperlink_found|hyperlink_kind|hyperlink_match|hyperlink_regex_searches|hyperlink_tooltip|hyperlink_word|hyperlinked|hyperlinkkind|hyperloglog|hyperloglogplus|hyperparametersf16invalid|hyperparameterswriteerror|hyperparamters|hyperpump|hyperrustls|hypersphere|hyperspheres|hyperthreading|hyph|hyphe|hyphen_marker|hypot|hypotenuse|hypotheticalbuilder|hypotheticalchunker|hypotheticalchunkererror|hypsiglena|hypthesis|hyrbkftcdj5crufeti6x26cj7rzlne32weugk7tlcwb8|hywsu4)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
