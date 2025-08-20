use regex::Regex;

pub fn matches_fc(text: &str) -> bool {
    let pattern = r"^(fc1_g|fc1_x|fc3gaugh2md1se3kkhpnlmkpqciard2psdgf7b2fds2n|fc_in|fc_messages|fc_messages_str|fc_out|fcall|fcc2huvrc7dv4ggehtziaremzrvwdw5miyu8ahuu1rsgja|fcc2ymnspo0c4lnbr|fcgi|fcgs|fchmod|fclonefileat|fcma|fcmae_ft_in1k|fcmfo6o8rrqony4i91fev1b8tbxrbjz1bweyenjrvmjnqi|fcommand|fcos|fcount|fct|fctl|fcxx|fcy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
