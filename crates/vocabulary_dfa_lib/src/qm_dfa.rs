use regex::Regex;

pub fn matches_qm(text: &str) -> bool {
    let pattern = r"^(qm|qm_allowed|qmark_test|qmarkoption|qmarkresult|qmatmulcomputenodekey|qmatrixinput|qmc|qmdiq5zpctyfhchybzzrs81ugit2qrnce2|qmistral|qmm_b_cpu|qmm_b_cuda|qmm_b_metal|qmm_batch|qmm_cpu|qmm_cuda|qmm_metal|qmm_n_cpu|qmm_n_cuda|qmm_n_metal|qmode|qmps1md6qyyfdpr23kuwu3tkgabwmxbdoqk|qmur929bbkkivjfrvyxnb90rbh2btjndjaxpelcvqwk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
