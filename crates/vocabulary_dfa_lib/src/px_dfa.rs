use regex::Regex;

pub fn matches_px(text: &str) -> bool {
    let pattern = r"^(px_12|px_16|px_8|px_neg_0p5|px_neg_1|px_with_ui_font_fallback|pxd|pxi|pxrmfvrjwockqeg|pxscale|pxypcpy6rbtrttw7phkcckrpp0yvhp5hdeickr6pllvdbfolx9qusycov0wzfjijnlgeysdlljizhhbn2mujvsahqqzetyp81efzlqnnpht4evvuh7vfdesu84kezmd5qlwpxlmvu31)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
