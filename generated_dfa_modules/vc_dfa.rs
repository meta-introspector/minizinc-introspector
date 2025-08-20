use regex::Regex;

pub fn matches_vc(text: &str) -> bool {
    let pattern = r"^(vceqq_s8|vcltzq_s8|vcpus|vcs_contents|vcs_file_collision|vcs_info|vcs_info_file|vcs_status_check_for_each_workspace_member|vcsinfo|vcvtq_f32_s32|vcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
