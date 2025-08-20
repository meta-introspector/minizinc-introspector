use regex::Regex;

pub fn matches_vf(text: &str) -> bool {
    let pattern = r"^(vfat|vfmaq_f32|vfoo|vfp|vfp2|vfp3|vfp4|vfr|vfs_config_version|vfs_def_file|vfs_extraincludes|vfs_load|vfs_notify|vfs_progress_config_version|vfs_read|vfs_span|vfsloader|vfspathrepr|vfspaths)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
