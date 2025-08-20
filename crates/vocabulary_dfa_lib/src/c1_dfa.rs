use regex::Regex;

pub fn matches_c1(text: &str) -> bool {
    let pattern = r"^(c14n0|c14n1|c14n2|c14n3|c14n4|c14n_nquads|c14nerror|c14nidmap|c14nquads|c14nstate|c14nterm|c1536_20|c1_contact_info|c1_score|c1_underscore|c1jbrksqaoq|c1️⃣2️⃣3️⃣)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
