use regex::Regex;

pub fn matches_h2(text: &str) -> bool {
    let pattern = r"^(h21|h22|h264_parameter_set_at_index|h264_parameter_set_count|h2_builder|h2_to_io_error|h2b|h2connectclient|h2h3|h2q|h2streamreadhalf|h2streamwritehalf)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
