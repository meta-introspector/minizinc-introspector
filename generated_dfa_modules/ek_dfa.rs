use regex::Regex;

pub fn matches_ek(text: &str) -> bool {
    let pattern = r"^(ek|eke|ekjwojgkn0x9kaphhzh9p6adtnqi65mp2x8p1a|ekkor|eks|ekxdjpg1lcwvzxqvuk1nmvbxplffuy4zcmotlw9m8i|ekyjpt4ol7kprmeoadygngnkhtvwcxqj2mkwagv4kuu4)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
