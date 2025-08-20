use regex::Regex;

pub fn matches_rb(text: &str) -> bool {
    let pattern = r"^(rb5zaoibabsxyjkbshstuuck2h3zx7n|rba|rbac_tests|rback|rbarr|rbbi|rbbr|rbbrk|rbf|rbir|rbk|rbr|rbra|rbrac|rbrace|rbrack|rbrk|rbrke|rbrks|rbrksl|rbrksld|rbrkslu|rbw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
