use regex::Regex;

pub fn matches_lw(text: &str) -> bool {
    let pattern = r"^(lwa_alpha|lwinhttp|lwinspool|lwn|lws2_32|lwunsmzzmb7es1wcmhffffbkykkk8jabjobuuwtcpsyub)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
