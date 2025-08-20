use regex::Regex;

pub fn matches_wy(text: &str) -> bool {
    let pattern = r"^(wylgwyipolsnhynedgeqmq4sglrkarlwcnkmbyv1aaxsbw4li3os4fpwxvpdxhcbebydtvxisbtuvkkq99nbsilnsrfstvb0alrdzruktdwphtn1v9hagfqmecx|wynn|wyoming)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
