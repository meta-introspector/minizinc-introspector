use regex::Regex;

pub fn matches_np(text: &str) -> bool {
    let pattern = r"^(np1|npa|npanics|npar|npara|nparagraph|nparal|nparall|nparalle|nparallel|nparent|npars|nparsed|nparsl|npart|npartial|npast|npat|npath|npatterns|npdf|nperforming|nperhaps|nplatform|nplot|nplus_one|npm_access|npm_call|npm_file|npm_install_package|npm_output|npm_release|npm_release_job|npm_token|npm_workflow|npminfo|npminfodisttags|npo|npoetic|npoetry|npoints|npol|npoli|npolin|npolint|npos|nppp|npppp|npq|npqrs|npqrst|npr|nprc|nprcu|nprcue|npre|nprec|nprece|npreceq|npreds|nprefix|nprint|nprobes|nproc|nproceeding|nprocesses|npropagated|nproxy|npubkey|npy_magic_string|npy_suffix)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
