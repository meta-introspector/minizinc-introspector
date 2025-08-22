use grex::RegExpBuilder;

pub fn generate_generalized_regex(lines: &[String]) -> String {
    let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let regex = RegExpBuilder::from(&line_refs).build();
    regex
}