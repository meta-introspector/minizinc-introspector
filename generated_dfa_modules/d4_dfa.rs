use regex::Regex;

pub fn matches_d4(text: &str) -> bool {
    let pattern = r"^(d4jsdcxaqdw8tdawn8h4r25cdns2ywlneujsl1zvjw6r|d4️⃣5️⃣6️⃣)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
