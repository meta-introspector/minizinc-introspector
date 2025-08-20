use regex::Regex;

pub fn matches_hg(text: &str) -> bool {
    let pattern = r"^(hg_ignore|hgemm_|hglobal|hgq9jf77xfxrgwrjy8vquhdbdugrt856rvqdzr1kjo6e|hgrepo|hgroup|hgtbqhacxntbpgplb2knjqwswpjyb2dqdb66lc3ph4an)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
