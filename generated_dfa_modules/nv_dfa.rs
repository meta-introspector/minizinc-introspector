use regex::Regex;

pub fn matches_nv(text: &str) -> bool {
    let pattern = r"^(nv12buffer|nv2|nva|nvalid|nvalidator|nvalue|nvap|nvar_from_env|nvariable|nvariadic|nvcc|nvd|nvdas|nvdash|nvembed_model|nversions|nvg|nvge|nvgt|nvh|nvha|nvhar|nvharr|nvi|nvim_buffer|nvim_mode_text|nvim_rs|nvim_window|nvimhandler|nvin|nvinf|nvinfi|nvinfin|nvisit|nvl|nvla|nvlar|nvlarr|nvle|nvlt|nvltr|nvltri|nvltrie|nvm|nvote|nvotes|nvptx|nvptx64|nvr|nvra|nvrar|nvrarr|nvrt|nvrtc|nvrtr|nvrtri|nvrtrie|nvs|nvsi|nvsim|nvvvv|nvwx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
