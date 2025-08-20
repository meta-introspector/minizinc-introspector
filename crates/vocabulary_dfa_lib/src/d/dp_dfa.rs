use regex::Regex;

pub fn matches_dp(text: &str) -> bool {
    let pattern = r"^(dp_size|dpath|dpb|dpb2|dpi_factor|dpi_x|dpi_y|dpimode|dpix|dpiy|dpjrepyumz5ndfu6h3wtqsqufsxafw8u7xqmwtewjdcp|dpms|dpmsolverplusplus|dpn|dprlyawmd|dpt|dpthead)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
