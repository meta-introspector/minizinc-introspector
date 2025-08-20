use regex::Regex;

pub fn matches_gs(text: &str) -> bool {
    let pattern = r"^(gs_body|gs_id|gs_storage_uri|gsbk|gsc|gscr|gse_equiv_impl|gserviceaccount|gsharp|gsi|gsids|gsim|gsime|gsiml|gsma|gso|gsp|gspomatchingiterator)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
