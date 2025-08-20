use regex::Regex;

pub fn matches_fk(text: &str) -> bool {
    let pattern = r"^(fk3ylwjcfdfyyso|fkacevngsy79rpqspnuv5gdyumoph4cehquxyfm8b8ap|fke75t4lxxgaqnvhdukm6dsfifvvragz8lyno7opwy1z|fku1qywlqsiehz644h6si65u5zq2cp9gxsyfufycuadv|fkzrycfecc8e48jp9ksw4xm77quv1bprdemktpcexwsa)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
