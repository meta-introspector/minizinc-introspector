use regex::Regex;

pub fn matches_r2(text: &str) -> bool {
    let pattern = r"^(r224_in1k|r25r24|r26|r27|r28|r29|r2_scores|r2d2|r2d2_sqlite|r2pvsm8snqxyz6fd2lbc9knrfdxmjj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
