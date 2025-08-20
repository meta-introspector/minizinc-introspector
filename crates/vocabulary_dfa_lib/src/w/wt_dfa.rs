use regex::Regex;

pub fn matches_wt(text: &str) -> bool {
    let pattern = r"^(wt2|wt_session|wte_idx|wtej|wtes|wtf8|wtplw6bmukhagbug7x7dmrfw|wtrait|wts0z3jdf3qwwxbbtjbvtvhdr8fmcfhdcwiqfm9xlerypknu9qhvx9k87|wtxn|wtype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
