use regex::Regex;

pub fn matches_qs(text: &str) -> bool {
    let pattern = r"^(qs_allinput|qs_filter_map_iter|qs_filter_map_to_quads|qs_filter_map_to_triples|qs_filter_triples|qs_map_iter|qs_map_to_quads|qs_map_to_triples|qs_to_triples|qsbiawdnzxigdgvzdcbjigd1zxnzcnnwyw5uaw5nig9uig11bhrpcgxligxpbmvzcmhvcgluzyb0aglzihdpbgwgd29yaw|qsc|qscr|qself1|qself2|qseq|qsfb9yukdjwuulx|qstablelm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
