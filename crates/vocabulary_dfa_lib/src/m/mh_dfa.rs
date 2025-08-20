use regex::Regex;

pub fn matches_mh(text: &str) -> bool {
    let pattern = r"^(mha_params|mha_with_lora|mhca|mhccaqeeil1ybmbwx|mhcefcsk8fnjj296ygvgdnunjhvkic4|mho|mhqcaqeeiagy7nzecvhkq4z1kdqby8swyaiykdqmtbehtim|mhqcaqeeijb2c89bvmjergnt|mhz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
