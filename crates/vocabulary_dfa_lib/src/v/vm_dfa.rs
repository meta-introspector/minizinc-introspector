use regex::Regex;

pub fn matches_vm(text: &str) -> bool {
    let pattern = r"^(vm_addr_end|vm_addr_start|vm_address_t|vm_base_address|vm_copy|vm_ctx|vm_data_addr|vm_deallocate|vm_end|vm_id|vm_instance|vm_key_addr|vm_lamports_addr|vm_map_t|vm_match_dynamic_super_instruction|vm_match_loop_function|vm_name|vm_offset_t|vm_owner_addr|vm_receiver|vm_region|vm_sender|vm_size_t|vm_summary|vm_to_host|vmaddress|vmap|vmconfig|vme|vmem|vmeta|vmlaq_n_f32|vmmemorypool|vmname|vmovdqu|vmovl_s8|vmovl_u8|vmrbsycv|vmrv3lyduw4vjjclqk9sty7pd2f4r6lz1p3fbmodbdp|vmsize_url|vmslice|vmslices|vmull_s16|vmull_s8|vmvalue|vmw_backdoor)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
