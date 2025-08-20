use regex::Regex;

pub fn matches_pn(text: &str) -> bool {
    let pattern = r"^(pn_char_base|pn_chars|pn_local|pnet_base|pnet_datalink|pneumonoultramicroscopicsilicovolcanoconiosis|pnew_info_opt|png__mime|png_bytes|png_header|png_mime_is_correct|png_path|png_path_str|pngdecoder|pngencoder|pnm|pno|pnode|pnu9xu9zrrs7bivrtn3ecnipj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
