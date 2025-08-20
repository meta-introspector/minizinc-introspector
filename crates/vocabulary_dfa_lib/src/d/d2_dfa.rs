use regex::Regex;

pub fn matches_d2(text: &str) -> bool {
    let pattern = r"^(d2_in1k|d2_path|d2aip4bbr8npwtu9vlrwrbvbuaq8w1zv38zflxx4pfbv|d2d1_alpha_mode|d2d1_alpha_mode_premultiplied|d2d1_alpha_mode_straight|d2d1_color_bitmap_glyph_snap_option_default|d2d1_color_f|d2d1_factory|d2d1_factory_type_multi_threaded|d2d1_feature_level_default|d2d1_pixel_format|d2d1_render_target_properties|d2d1_render_target_type_default|d2d1_render_target_usage_none|d2d1_unit_mode_dips|d2d1createfactory|d2multisample|d2s)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
