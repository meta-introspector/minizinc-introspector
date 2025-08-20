use regex::Regex;

pub fn matches_wm(text: &str) -> bool {
    let pattern = r"^(wm8z5avl2cdyaly6ze7kvw0s64utwtumbavvskli|wm_activate|wm_base|wm_change_state|wm_char|wm_close|wm_create|wm_deadchar|wm_debug_display|wm_default|wm_delete_window|wm_destroy|wm_displaychange|wm_dpichanged|wm_entermenuloop|wm_entersizemove|wm_exitmenuloop|wm_exitsizemove|wm_for_testing|wm_getminmaxinfo|wm_gpui_close_one_window|wm_gpui_cursor_style_changed|wm_gpui_dock_menu_action|wm_gpui_task_dispatched_on_main_thread|wm_hint_property_state|wm_ime_composition|wm_ime_startcomposition|wm_inputlangchange|wm_job_updated|wm_keydown|wm_keyup|wm_lbuttondown|wm_lbuttonup|wm_mbuttondown|wm_mbuttonup|wm_mousehwheel|wm_mouseleave|wm_mousemove|wm_mousewheel|wm_move|wm_name|wm_nccalcsize|wm_nccreate|wm_ncdestroy|wm_nchittest|wm_nclbuttondown|wm_nclbuttonup|wm_ncmbuttondown|wm_ncmbuttonup|wm_ncmouseleave|wm_ncmousemove|wm_ncrbuttondown|wm_ncrbuttonup|wm_paint|wm_protocols|wm_quit|wm_rbuttondown|wm_rbuttonup|wm_setcursor|wm_size|wm_size_hints|wm_syscommand|wm_syskeydown|wm_syskeyup|wm_terminate|wm_timer|wm_value|wm_xbuttondown|wm_xbuttonup|wma|wmcapabilities|wmctrl|wmem_max|wmhintpropertystate|wmi|wmsizehints|wmul_impl|wmv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
