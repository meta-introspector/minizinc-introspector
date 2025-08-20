use regex::Regex;

pub fn matches_tg(text: &str) -> bool {
    let pattern = r"^(tg6|tga|tgbjmh9pw84rqjm8bnmguubzuwid4nh1xdjti|tghi|tgi|tgmath|tgraph|tgt_crate|tgt_kind|tgt_name|tgt_pat|tgt_range|tgzip)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
