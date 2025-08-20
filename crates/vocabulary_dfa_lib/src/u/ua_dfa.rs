use regex::Regex;

pub fn matches_ua(text: &str) -> bool {
    let pattern = r"^(ua3n2wnm|uaa|uaaa|uaaaa|uaaaaaaaaaaa|uaaaauaa|uaaau|uabs|uac|uaccum|uacu|uacut|uacute|uadd|uae|uae_large_v1|uapi|uar|uarr|uarray|uarro|uarroc|uarroci|uarrocir|uasa|uaua|uauaa|uauaaa|uauau|uauaua|uauauaua|uaux)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
