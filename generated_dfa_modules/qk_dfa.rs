use regex::Regex;

pub fn matches_qk(text: &str) -> bool {
    let pattern = r"^(qk4_0|qk4_1|qk5_0|qk5_1|qk8_1|qk_dtype|qk_layer_norm|qk_nope_head_dim|qk_rope_head_dim|qk_val|qkk2sd3i4io1acrxpcmcx7tc5mypozmkjcjhyysbla9cgmuorvzdla|qknorm|qkv2|qkv_attention|qkv_b|qkv_clip|qkv_hidden_size|qkv_out_dim|qkv_w|qkvonlyattnprojections|qkvonlyditblock|qkvs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
