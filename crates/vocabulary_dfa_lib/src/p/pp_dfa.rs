use regex::Regex;

pub fn matches_pp(text: &str) -> bool {
    let pattern = r"^(pp_state|ppad|pparameter|ppatch|ppincludedfile|ppm|ppmm|ppp|ppp_x|pprint_ast|pprofprofiler|pprove|ppt|pptx|ppvobject|ppx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
