use regex::Regex;

pub fn matches_fg(text: &str) -> bool {
    let pattern = r"^(fg12tb1tz8w6zjsq4zagotwocztdmjf9hqk8r11pakog|fgh|fghijkl|fghiklmnopqrstuvwxyz|fgnuc|fgyh8eeygztbw8ss33ymnqnzx54wxprj5kwnpkcfwpot)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
