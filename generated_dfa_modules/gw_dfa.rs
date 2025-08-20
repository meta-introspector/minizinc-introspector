use regex::Regex;

pub fn matches_gw(text: &str) -> bool {
    let pattern = r"^(gw6s9cpzr8jhku1qqmdiqcmukjc2dhj3gzagwdua6pgw|gw7s9cpzr8jhku1qqmdiqcmukjc2dhj3gzagwdua6pgw|gw_addr|gw_name|gwitness|gwl_exstyle|gwl_style|gwlp_hinstance|gwlp_userdata|gwtdqbghctbgmx2cpegnpxtebutqradmgtr5qychdgmj|gwtx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
