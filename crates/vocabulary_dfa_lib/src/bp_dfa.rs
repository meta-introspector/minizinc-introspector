use regex::Regex;

pub fn matches_bp(text: &str) -> bool {
    let pattern = r"^(bp_gens|bp_in_file|bp_prompt|bparent|bpaths|bpe_tokenizer|bpe_tokens|bpebuilder|bpf4erpevsfmcstnh1pztwtkalrkxvmxedthxhuwcqcf|bpf_l|bpf_loader_instruction|bpf_loader_program_account|bpf_loader_program_id|bpf_loader_size|bpf_loader_state|bpf_loader_upgradeable_id|bpf_out_dir|bpf_program_id|bpf_target_feature|bpf_upgradeable_loader_instruction|bpf_upgradeable_loader_program_id|bpfallocator|bpfexecutor|bpfprogram|bpfupgradeableloader|bpfupgradeableloaderaccounttype|bpg|bplaced|bpple|bpr|bpri|bprim|bprime|bpt2lufektfhi|bpxrficyexamplekey)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
