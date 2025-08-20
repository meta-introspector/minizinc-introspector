use regex_automata::{meta::Regex, PatternID};

pub fn parse_key_value_pairs(hay: &str) -> Result<Vec<(&str, &str)>, Box<dyn std::error::Error>> {
    let re = Regex::new_many(&[
        r#"(?m)^(?<key>[[:word:]]+)=(?<val>[[:word:]]+)$"#,
        r#"(?m)^(?<key>[[:word:]]+)="(?<val>[^"]+)"$"#,
        r#"(?m)^(?<key>[[:word:]]+)='(?<val>[^']+)'$"#,
        r#"(?m)^(?<key>[[:word:]]+):\s*(?<val>[[:word:]]+)$"#,
    ]).map_err(|e| format!("Failed to compile regex: {}", e))?;

    let mut kvs = vec![];
    for caps in re.captures_iter(hay) {
        let key = caps.get_group_by_name("key")
            .ok_or("Key capture group not found")?;
        let val = caps.get_group_by_name("val")
            .ok_or("Value capture group not found")?;
        kvs.push((&hay[key.range()], &hay[val.range()]));
    }
    Ok(kvs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_value_pairs() {
        let hay = r#"
best_album="Blow Your Face Out"
best_quote="\"then as it was, then again it will be\""
best_year=1973
best_simpsons_episode: HOMR
"#;
        let kvs = parse_key_value_pairs(hay).unwrap();
        assert_eq!(kvs, vec![
            ("best_album", "Blow Your Face Out"),
            ("best_quote", "\"then as it was, then again it will be\""),
            ("best_year", "1973"),
            ("best_simpsons_episode", "HOMR"),
        ]);
    }
}
