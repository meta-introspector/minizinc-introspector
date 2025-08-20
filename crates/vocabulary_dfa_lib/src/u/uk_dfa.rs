use regex::Regex;

pub fn matches_uk(text: &str) -> bool {
    let pattern = r"^(ukovkoftjo9sfg05arbezj2qe0whlvykqayvpznbvyrk53nvrxdblxwqi1mr5nolfvkq0shh1z2bkuymbng5zjilt29hvjm1l3a|ukplab|ukysscozcv71mmkhrnqfpp0ke1unet5ay4oaogccqgsm49)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
