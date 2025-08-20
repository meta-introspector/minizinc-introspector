use regex::Regex;

pub fn matches_cb(text: &str) -> bool {
    let pattern = r"^(cb11czrjwuoihyvgfucbvg1vr4qtnfx65ss4jvk1h0q|cb_cr_texture|cb_data|cb_delta|cb_dim|cb_idx|cb_meta|cb_size|cb_start|cb_stop|cbar|cbaz|cbcrtexture|cbin_expr|cbkdrordqm8hwhe6ak9cgupjuomrasekfmxeaz5cnnxz|cblas|cbor_rpc|cbor_value_to_value|cbp|cbrace_end|cbrace_start|cbrt|cbut|cbuttons)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
