use regex::Regex;

pub fn matches_nl(text: &str) -> bool {
    let pattern = r"^(nl0tcxcmllwy9icxxqmq|nl_fence_pattern|nl_translator|nla|nlambda|nlar|nlarr|nlast|nlayers|nlcf|nldr|nle|nleader|nlearn|nlef|nlefta|nleftar|nleftarr|nleftarro|nleftarrow|nleftr|nleftri|nleftrig|nleftrigh|nleftright|nleftrighta|nleftrightar|nleftrightarr|nleftrightarro|nleftrightarrow|nleq|nleqq|nleqs|nleqsl|nleqsla|nleqslan|nleqslant|nles|nless|nless_flat|nlgn|nlibrary|nlifetime|nlimit|nline6|nline7|nlines|nlink|nlinks|nlint|nlittle|nll_and_cross_entropy|nll_loss|nlll|nllll|nlm|nlmo|nlmˇo|nloaded|nlocal|nlog|nlol|nlongest|nlots|nlp_token_graph|nlprule|nlpruleplugin|nlptoken|nlptokengraph|nlptokenindex|nlptokenspan|nls|nlsi|nlsim|nlt|nltr|nltri|nltrie|nltv|nly|nlíne)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
