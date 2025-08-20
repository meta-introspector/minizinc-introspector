use regex::Regex;

pub fn matches_wp(text: &str) -> bool {
    let pattern = r"^(wp2|wp_cursor_shape_device_v1|wp_cursor_shape_manager_v1|wp_fractional_scale_manager_v1|wp_fractional_scale_v1|wp_viewport|wp_viewporter|wpcursorshapedevicev1|wpcursorshapemanagerv1|wpe|wpfractionalscale|wpfractionalscalemanagerv1|wpfractionalscalev1|wpos_ids|wps5x1mou5pudpd|wpviewport|wpviewporter)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
