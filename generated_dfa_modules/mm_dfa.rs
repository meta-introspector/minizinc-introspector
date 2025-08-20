use regex::Regex;

pub fn matches_mm(text: &str) -> bool {
    let pattern = r"^(mm1|mm256_set_m128i|mm3|mm4|mm_bf16_reduced_precision|mm_f16_reduced_precision|mm_f32_reduced_precision|mm_height|mm_hidden_size|mm_input_start|mm_layout|mm_layout_cpu|mm_layout_gpu|mm_layout_metal|mm_patch_merge_type|mm_per_inch|mm_projector|mm_projector_type|mm_rodata_start|mm_use_im_start_end|mm_vision_select_feature|mm_vision_select_layer|mm_vision_tower|mm_width|mmacosx|mmap1|mmap2|mmap_arc|mmap_arc_obj|mmap_cache|mmap_directory_tests|mmap_opt|mmap_populate|mmap_specific|mmap_us|mmap_weak|mmap_weakref|mmaparc|mmapasrawdesc|mmapcache|mmapcompatibleloader|mmapdirectoryinner|mmapedfile|mmappable|mmapping|mmastrac|mmax|mmdbmo2l|mmdit_config|mmditconfig|mmditcore|mmditjointblock|mmditx|mmditxjointblock|mmlu|mmlu_dir|mmmap|mmmmdddd|mmmmmmmm|mmprojector|mmq_x|mmq_x_q4_0_ampere|mmq_y|mmq_y_q4_0_ampere|mmsg|mmsghdr|mmsghdr_for_packet|mmss|mmultiscripts|mmut|mmvq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
