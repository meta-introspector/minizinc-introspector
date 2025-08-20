use regex::Regex;

pub fn matches_dl(text: &str) -> bool {
    let pattern = r"^(dl_cur|dl_info|dl_path|dl_retry_multiple|dl_retry_single|dl_total|dl_url|dladdr|dlc|dlclose|dlcloseunknown|dlco|dlcor|dlcorn|dlcr|dlcro|dlcrop|dlerror_str|dli_fbase|dli_fname|dli_saddr|dli_sname|dlinfo|dll_instance|dll_process_attach|dllexport|dllgetclassobject|dllimport|dlmopen|dlopen2|dlopenunknown|dloss|dloss_dy|dlp|dlpldqrcl5zwb0eajcmebalja6rrzkpmsypdvmjdom0|dlsymunknown|dlt|dlt_threshold|dlv|dlv_path|dlvflags)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
