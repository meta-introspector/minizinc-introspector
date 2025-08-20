use regex::Regex;

pub fn matches_g_(text: &str) -> bool {
    let pattern = r"^(g_|g_bar|g_bar_pub|g_bar_s|g_bar_s_pub|g_bar_struct|g_bar_struct_pub|g_def|g_factors|g_foo|g_foo_computed|g_foo_pub|g_foo_pub_crate|g_function|g_i|g_id|g_ij|g_in|g_k0|g_k1|g_l|g_linear|g_or_s|g_r|g_times_a_times_s|g_val|g_vec|g_wtx|g_x)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
