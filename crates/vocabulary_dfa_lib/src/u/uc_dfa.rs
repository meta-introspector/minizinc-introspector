use regex::Regex;

pub fn matches_uc(text: &str) -> bool {
    let pattern = r"^(ucanonical|ucase|ucd|uceil|ucg|uchar_ptr|ucinated|ucir|ucirc|ucirvine|uckeytranslate|ucopy_bf16|ucopy_f16|ucopy_f32|ucopy_f64|ucopy_i64|ucopy_u32|ucopy_u8|ucos|ucred|ucs|ucut|ucy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
