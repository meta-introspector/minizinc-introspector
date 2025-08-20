use regex::Regex;

pub fn matches_m3(text: &str) -> bool {
    let pattern = r"^(m30|m31|m313|m316|m317|m32|m325|m32s|m336|m345|m355|m372|m374|m382|m383|m392|m395|m3_1|m3_2|m3_3|m3b|m3lazmrzukxdc1q5mszfas_kjrwukz3joodmjj0g4gm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
