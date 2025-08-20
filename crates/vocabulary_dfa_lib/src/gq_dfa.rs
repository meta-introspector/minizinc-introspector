use regex::Regex;

pub fn matches_gq(text: &str) -> bool {
    let pattern = r"^(gqa_factor|gqaldac48fehzgwrj9il5q889emjkcj3acvhf7vcbbf4|gqjw8vvvskszwtje6x6mffhi7kcu6mqst8pf7493h2hk|gql|gqme27wmt4|gquark|gqxzc7yisnkje6ffuk6sc2p53xrvkoaz9vmktyzumnpl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
