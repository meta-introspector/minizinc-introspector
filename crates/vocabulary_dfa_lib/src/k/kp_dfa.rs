use regex::Regex;

pub fn matches_kp(text: &str) -> bool {
    let pattern = r"^(kp1|kp2|kp_|kp_connections|kp_next|kp_prior|kpis|kprime|kpt|kpyispwm2rkmidqf30fg1niy8xnkvasfepoca)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
