use regex::Regex;

pub fn matches_ei(text: &str) -> bool {
    let pattern = r"^(ei1odph6wtmlrplodudwtscrt|ei64max|ei_column|eib|eiffel|eig|eig_val|eig_vec|eigen|eigenspaces|eigenstatements|eigenvalue|eigenvectors|eigh_into|eighinto|eight_large_segments|eighteen|eighteenˇ|eightwothree|eightˇ|eigth|eigvals|eigvecs|eij|ein|eindex|eine|einem|einen|einer|eines|einig|einige|einigem|einigen|einiger|einiges|einmal|einstein|eip|eips|eisdir|eit|eitem|either2|either4|either5|either6|either7|either8|either_into|either_name_or_example|eitherproj|eitt|eiusmod|eivät)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
