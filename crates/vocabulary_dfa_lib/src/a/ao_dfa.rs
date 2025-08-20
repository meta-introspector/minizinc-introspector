use regex::Regex;

pub fn matches_ao(text: &str) -> bool {
    let pattern = r"^(aog|aogll|aogo|aogon|aom|aomedia|aopf|aos|aout|aox4ywrr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
