use regex::Regex;

pub fn matches_kf(text: &str) -> bool {
    let pattern = r"^(kf|kf_flag_dont_verify|kf_path|kf_structsize|kflfr09iah4jipyghmjkuxctpqsqaffnemuht9pc7drnl4zkzqexfxhsmcaaaaaak9rwgobudaycqom9u36vs0s6ighpkhr0z|kfox|kfr|kfseventstreamcreateflagfileevents|kfseventstreamcreateflagnodefer|kfseventstreamcreateflagwatchroot|kfx1olpfc5cwmfesgxrncxaoukjfkpusj4uskb3zkmx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
