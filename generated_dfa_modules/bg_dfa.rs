use regex::Regex;

pub fn matches_bg(text: &str) -> bool {
    let pattern = r"^(bg_edit_files_disclosure|bg_entry|bg_eof_reached|bg_event|bg_executor|bg_exit|bg_left|bg_luminance|bg_reader_data|bg_reader_join_handle|bg_right|bg_rx|bg_task|bg_thread|bg_throttling_wait_percent|bg_throttling_wait_us|bg_waiting_percent|bg_waiting_us|bg_y|bg_y_clamped|bgcolor|bge_base_en|bge_base_en_v1_5|bge_large_en|bge_large_en_v1_5|bge_small_en|bgebaseenv15|bgebaseenv15q|bgebasesmall|bgelargeenv15|bgelargeenv15q|bgelargezhv15|bgemm|bgererankerbasev2|bgererankerlarge|bgererankerv2m3|bgesmallenv15q|bgesmallzhv15|bgn3e|bgp_rec|bgpiterator|bgr|bgr0|bgr8_to_yuv420|bgr_data|bgr_index|bgra8|bgra8unorm|bgra8unormsrgb|bgrx|bgsound|bgthreads|bgurgqqacg|bgurgqqahg|bgvytjefmzydvkiptmmjxgzv8iqoo4mwjsp3qstkhhxa|bgygbgygbgygbgygbgyaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan1rczlfao|bgˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
