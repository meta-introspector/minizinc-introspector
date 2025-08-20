use regex::Regex;

pub fn matches_uf(text: &str) -> bool {
    let pattern = r"^(ufcs_function_calls|ufcs_matches_method_call|ufcscallinfo|ufe|ufi|ufis|ufish|ufisht|uflgjs|ufloor|ufncgduhyampkuxumsvnmkf5bip4nr3x18cr9khtazbrkv6erjaxickyrnacazgex|ufr|uft)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
