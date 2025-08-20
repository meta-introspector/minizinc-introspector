use regex::Regex;

pub fn matches_hb(text: &str) -> bool {
    let pattern = r"^(hb_port|hba|hbar|hbc|hbits|hbone_addr|hbone_address|hbone_mtls_port|hbone_pool|hboneconnectioninfo|hbrbackground|hbs_actual|hbs_name|hbs_template|hbs_test|hbz5ffmkwnhc7uwk6tf1hvi6tcs7dtyfdjecupggzfag)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
