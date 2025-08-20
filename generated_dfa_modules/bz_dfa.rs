use regex::Regex;

pub fn matches_bz(text: &str) -> bool {
    let pattern = r"^(bzbbveudymeyoyzcmwnqcx3cd4jqs7puavfhltsbb6fm|bzcuf0vaqbqa4subzbo9z3rwimlyibaz0bqzgjmtpofufmnycjaeb29vlceafybn5gdlgtwp50tkmhoskndulzikzjz4qhsa9rbg2ztomhocsaobhku2h9oxcak4scj1qgwst|bzero|bzl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
