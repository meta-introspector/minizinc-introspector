use regex::Regex;

pub fn matches_l4(text: &str) -> bool {
    let pattern = r"^(l405|l409|l41|l415|l422|l425|l433|l437|l439|l44|l446|l448|l452|l4561|l461|l462|l476|l481|l482|l489|l490|l491|l499)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
