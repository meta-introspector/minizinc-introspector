use regex::Regex;

pub fn matches_lu(text: &str) -> bool {
    let pattern = r"^(lu_decomposition|lua_language|luau|lub|lucky_seven|luckyentry|luctus|lue|lufs|luma_ray_v2_0|lumbermill|luminance_bytes|luminance_stride|luminances|lumpimupli|lunar|lung|lupus|lur|lurd|lurds|lurdsh|lurdsha|lurdshar|lurkers|luru|luruh|luruha|luruhar|luser32|luserenv|lust|lustre|luxembourgish)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
