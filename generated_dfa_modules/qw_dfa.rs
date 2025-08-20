use regex::Regex;

pub fn matches_qw(text: &str) -> bool {
    let pattern = r"^(qw|qwen1_5_0_5b_chat|qwen1_5_110b_chat|qwen1_5_14b_chat|qwen1_5_1_8b_chat|qwen1_5_32b_chat|qwen1_5_4b_chat|qwen1_5_72b_chat|qwen1_5_7b_chat|qwen2_5|qwen2_5_72b_instruct_turbo|qwen2_5_7b_instruct_turbo|qwen2_5_coder|qwen2_5visionpatchembed|qwen2_vl|qwen3attention|qwen3config|qwen3feedforward|qwen3mlp|qwen3mlpexpert|qwen3rotaryembedding|qwen3sparsemoeblock|qwen7b|qwen_2_1_5b_instruct|qwen_2_5_32b_vl_chat_f16|qwen_2_5_32b_vl_chat_q4|qwen_2_5_32b_vl_chat_q8|qwen_2_5_3b_instruct|qwen_2_5_3b_vl_chat_f16|qwen_2_5_3b_vl_chat_q8|qwen_2_5_72b|qwen_2_5_7b_instruct|qwen_2_5_7b_vl_chat_f16|qwen_2_5_7b_vl_chat_q4|qwen_2_5_7b_vl_chat_q8|qwen_2_5_coder_32b|qwen_2_72b_instruct|qwen_2_7b_instruct|qwen_eps|qwen_image_processing|qwen_patch_merger|qwen_qvq_preview|qwen_qwq_32b|qwen_qwq_preview_32b|qwen_rope|qwen_tokenizer|qwen_vision|qwen_vision_block|qwen_vision_embed|qwenvl|qwenvlropecache|qwone|qwqwq|qwxhzgrpbjpvcgvuc2vzyw1l)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
