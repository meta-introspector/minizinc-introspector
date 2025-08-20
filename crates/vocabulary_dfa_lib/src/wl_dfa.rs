use regex::Regex;

pub fn matches_wl(text: &str) -> bool {
    let pattern = r"^(wl_buffer|wl_callback|wl_compositor|wl_data_device|wl_data_device_manager|wl_data_device_manager_version|wl_data_offer|wl_data_source|wl_key|wl_keyboard|wl_output|wl_output_max_version|wl_output_min_version|wl_output_version|wl_pointer|wl_region|wl_registry|wl_seat|wl_seat_max_version|wl_seat_min_version|wl_seat_version|wl_shm|wl_shm_pool|wl_surface|wlayernorm|wlbuffer|wlcallback|wlckv1a64ngtckprgu4s4grvtestxjmnjxbjakzracn|wlcompositor|wldatadevice|wldatadevicemanager|wldataoffer|wldatasource|wlfp|wlkeyboard|wloutput|wlpointer|wlregion|wlregistry|wlseat|wlshm|wlshmpool|wlsurface)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
