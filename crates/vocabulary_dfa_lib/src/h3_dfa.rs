use regex::Regex;

pub fn matches_h3(text: &str) -> bool {
    let pattern = r"^(h3_graph|h3_section|h3kbsakdeiusyhmehqjjync27jesxz6zwj3zwkowqbkv|h3ni7vg1csmjzdtvxf7rkaf9um5qk4rsohjsmpvtznnu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
