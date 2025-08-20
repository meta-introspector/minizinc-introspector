use regex::Regex;

pub fn matches_kq(text: &str) -> bool {
    let pattern = r"^(kq|kq_scale|kq_scaled|kqe|kqueuewatcher|kqv|kqv_merged|kqv_merged_contiguous|kqz9zzjmrhccudnxxkmupsjwkrousjkeo7hrug2kfhah85vckxprna4r9qph7tf2wvbtd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
