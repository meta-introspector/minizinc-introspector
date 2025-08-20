use regex::Regex;

pub fn matches_km(text: &str) -> bool {
    let pattern = r"^(kmask1|kmask2|kmask3|kmditemcfbundleidentifier|kmeansinit|kmeansparams|kmeansparamserror|kmeansplusplus|kmeansvalidparams|kmerged|kmhf)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
