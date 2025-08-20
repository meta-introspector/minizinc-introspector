use regex::Regex;

pub fn matches_oe(text: &str) -> bool {
    let pattern = r"^(oe4|oe8z7bbv2ahnjxebk6asjfkjbolyqdnn6zpkg2u4sna|oeis_fibonacci|oeis_penrose_patterns|oeis_sequence|oeisreflection|oeiwcerrrt9atllkmvwvitzbhn25yomod0kfgymtbvaw86teryfx4tu98blynuqtthf9vxcu8xy0rfacxmus7lhbp|oel|oeli|oelig|oem|oem5gvefgggkg6g67t9dhkjtos0y|oer|oerators|oever)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
