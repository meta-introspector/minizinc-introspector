use regex::Regex;

pub fn matches_gf(text: &str) -> bool {
    let pattern = r"^(gf6249277|gfefwuzkp9nmap5u4vfnfgevqoeqc2wpgnbfruzhpib5|gfile_id|gflops|gfni|gfp|gfr|gfunc|gfunc_cube|gfunc_logcosh|gfx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
