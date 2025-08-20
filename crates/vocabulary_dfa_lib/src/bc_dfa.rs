use regex::Regex;

pub fn matches_bc(text: &str) -> bool {
    let pattern = r"^(bcast|bcast_dims|bcast_lhs|bcast_ndims|bcast_rhs|bcast_value|bccwj|bcdmatchingiterator|bchildren|bchildren1|bcksp|bclabels|bcmatchingiterator|bco|bcojqvof5fu1knyfskwkcaqb4cgddpxb7|bcon|bcong|bcow|bcwknvcgvonn8sl4he4xfuevgfcee5mwxwpagp6zv89x|bcy|bczfxd1nwzaoxsdpgvsmerobyfnqltv9|bcˇδˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
