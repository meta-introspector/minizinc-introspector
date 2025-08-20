use regex::Regex;

pub fn matches_uv(text: &str) -> bool {
    let pattern = r"^(uv_height|uv_idx|uv_index|uv_size|uv_status|uv_width|uvicorn|uvmjbmkyfkkvdsmqzabyb0ytechh7apunro|uvuoszbb7ifbcknqlilgdjbxnpa7msil1blz0qesumapiuwhkkajizjjb1qjlswyrpxepkwm23an5cvdfmy2tjfgk07bllvxn9uvciwmssg0skyojsvedyptijprbyrhx1ckgnghad8k5ehcy45ryqshx7trbtflisumooevkkvrkhj05j|uvwx|uvwxyz0123|uvx_malicious)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
