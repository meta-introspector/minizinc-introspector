use regex::Regex;

pub fn matches_qc(text: &str) -> bool {
    let pattern = r"^(qc|qcases|qckflhnjmq12xphyamg9bkg3w|qcommand|qcur|qcysxot)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
