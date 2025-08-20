use regex::Regex;

pub fn matches_qp(text: &str) -> bool {
    let pattern = r"^(qpath_certainty|qpath_res_collector|qpath_search_pat|qpath_span_without_turbofish|qpath_spans|qpktf|qpr|qpri|qprim|qprime|qproj|qps)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
