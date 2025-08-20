use regex::Regex;

pub fn matches_il(text: &str) -> bool {
    let pattern = r"^(il1|il2|il_1nddbo2ezvkylo2ckcxnp8j8|ilcpcyblfs5succewadj12zyb|ilen|ill_formed_attribute_input|illama|illconditioned|ille|illegal_mutable_capture_ids|illegalcast|illegalchar|illegalcharacter|illegally|illegalowner|illegalselftypevisitor|illetve|illinois|illuminate|illuminated|illuminates|illuminating|illustrating|illustrations|ilog|ils|ilyen|ilyenkor)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
