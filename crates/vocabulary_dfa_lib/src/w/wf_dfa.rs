use regex::Regex;

pub fn matches_wf(text: &str) -> bool {
    let pattern = r"^(wf|wfft_repr_alpha_inplace|wfi|wfr|wfxt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
