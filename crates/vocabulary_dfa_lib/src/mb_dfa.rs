use regex::Regex;

pub fn matches_mb(text: &str) -> bool {
    let pattern = r"^(mb0|mb1|mb2|mb5k0txajwcmvh1ofn9h3sglttucg|mb_0|mb_0p5|mb_10|mb_1p5|mb_2p5|mb_3|mb_4|mb_iconerror|mb_info|mb_systemmodal|mba|mbc|mbconv|mbconv_expand_ratio|mbe_are_not_attributes|mbe_smoke_test|mbes|mbindings|mbox|mbox_alice|mbpusdvjuxnjoauwbmw6oracq6tq9o1z|mbrubeck|mbufmod)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
