use regex::Regex;

pub fn matches_vb(text: &str) -> bool {
    let pattern = r"^(vb2|vb5e6|vb_clip_g|vb_clip_l|vb_fn|vb_h|vb_head|vb_prefix|vb_reference|vb_s|vb_t5|vb_vae|vb_w|vb_x|vba|vbar|vbarv|vbicq_u8|vbproj|vbs|vbscript|vbv4zrha0or9rgjfz2spt3gx2jyskdkraqgblozsgfqg9hsbz57i55sqfliuf5mbyzkuzgkhsihi81bdqbrfaj6x5boemaalqscboca5xgduz2hmpuguad9f|vbyte)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
