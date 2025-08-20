use regex::Regex;

pub fn matches_sz(text: &str) -> bool {
    let pattern = r"^(sz64|szdevice|szemben|szerint|szinte|szl|szli|szlig|szoid_ctl|szowhm8h9kj84ojqc1lezkjnvhinpgcxm8jhzpoabsdfl5fnfek5|számára)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
