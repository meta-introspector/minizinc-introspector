use regex::Regex;

pub fn matches_gm(text: &str) -> bool {
    let pattern = r"^(gm8pv0txkm|gmake|gmask|gmax|gmax1|gmax2|gmax3|gmax4|gmaxn1|gmaxn2|gmaxp1|gmaxp2|gmbh|gmc19j9qln2rfk5ndux6qxadhvpgncvvbzym8e9wmz2f|gmem_moveable|gmf|gmm_centroids|gmmcovartype|gmmerror|gmminitmethod|gmmparams|gmmvalidparams|gmodkind|gmonth|gmonthday|gmskl|gmt|gmubvtfb2ahfsfmxpufewzghydeclps79s48fmcwcfm5|gmyw1nqycrw7p7jqrcyp9ivu9hynbrgz1r5syjjh41fs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
