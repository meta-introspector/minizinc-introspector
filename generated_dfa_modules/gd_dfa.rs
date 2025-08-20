use regex::Regex;

pub fn matches_gd(text: &str) -> bool {
    let pattern = r"^(gday|gdb_path|gdb_script_file|gdbwire|gdh5tvdbtpuprnxaryqqikua7uzabz28q2n9bhbkomlm|gdiqx|gdjz7lskyjqqz6ujcaaqrjrmq7tlncwyjhdt84qt4qwk|gdm_session|gdmsession|gdn|gdnsyh3ytwcxfvqrvvjmm1jhts4qvx7mfsx56ujlufiz|gdo|gdot)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
