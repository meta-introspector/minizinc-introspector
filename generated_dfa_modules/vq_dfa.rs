use regex::Regex;

pub fn matches_vq(text: &str) -> bool {
    let pattern = r"^(vq_strides|vqaaa|vqgan|vqgan_weights|vqgan_weights_source|vqgan_weights_source_display)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
