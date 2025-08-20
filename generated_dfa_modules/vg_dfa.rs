use regex::Regex;

pub fn matches_vg(text: &str) -> bool {
    let pattern = r"^(vget_high_s16|vget_high_s8|vget_high_u8|vget_lane_u64|vget_low_s16|vget_low_s8|vget_low_u8|vgetq_lane_f32|vgg13|vgg13_blocks|vgg16|vgg16_blocks|vgg19|vgg19_blocks|vgomkporteevbclsalw6stgtfpcrimu3c)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
