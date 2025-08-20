use regex::Regex;

pub fn matches_rd(text: &str) -> bool {
    let pattern = r"^(rd0|rd_cb6|rdata|rdbg|rdc|rdca|rdf_first|rdf_first_id|rdf_json|rdf_language|rdf_list|rdf_lit|rdf_nil|rdf_obj|rdf_object|rdf_path|rdf_processing_lib|rdf_quads|rdf_quads_full|rdf_quads_with|rdf_rest|rdf_rest_id|rdf_subject|rdf_terms|rdf_ty|rdf_type_str|rdf_value_triples|rdf_value_with|rdflit|rdfliteral|rdfliteralref|rdfmetadata|rdfo|rdfobject|rdfp|rdfprocessingoptions|rdfproperty|rdfquad|rdfs_label_str|rdfsignal|rdfstatistics|rdfterms|rdftype|rdfxmlconfig|rdfxmlerror|rdfxmlformatter|rdfxmlserializer|rdim|rdist|rdl|rdld|rdldh|rdldha|rdldhar|rdm|rdmx6|rdnetworkkargs|rdoc|rdq|rdqu|rdquo|rdquor|rdr2|rds|rdsh|rdtscp|rdw_invalidate|rdw_updatenow|rdx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
