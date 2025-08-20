use regex::Regex;

pub fn matches_wc(text: &str) -> bool {
    let pattern = r"^(wc43uzxwb2kpmeqtuy1sgrlczqtasoetfkzi0aum7aedowe5atots4cqzhwimb1bmfpf4daq92cz8jhrm4rwetbo29wmjvicjea3knqyd3oa7h9ae9z|wc604ydgxa8vjis5ap43jxiuffaaq|wc_case_is_ok|wchar_t|wci|wcir|wcirc|wclippy|wcmwimlxbthzhbungpqj0etgjvutoweruhbiwdj|wconversion|wcount|wcs|wcslen|wctype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
