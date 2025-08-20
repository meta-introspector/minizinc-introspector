use regex::Regex;

pub fn matches_ew(text: &str) -> bool {
    let pattern = r"^(ew91igfyzq|ewer|ewind|ewme9ufqfy1ikk1jhjs8fm5hxwnk336qjpbscntizktu|ewresize|ewrwer|ewzht8n9mhpf2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
