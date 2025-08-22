use anyhow::Result;
use grex::RegExpBuilder;

#[allow(dead_code)]
pub fn handle_unmatched_regex_error(file_path: &std::path::PathBuf, error_message: &str) -> Result<()> {
    eprintln!("Error fixing {:?}: {}\n", file_path, error_message);

    let unmatched_line = error_message.trim_start_matches("No regex matched line: ");

    eprintln!("\nAttempting to generate regex for: `{}`", unmatched_line);
    let generated_regex = RegExpBuilder::from(&[unmatched_line])
        .build();

    eprintln!("Generated Regex: `{}`", generated_regex);
    eprintln!("Please review this regex and add it to `crates/poem_yaml_fixer/src/regex_patterns.toml`.");
    eprintln!("Example entry:");
    eprintln!("```toml");
    eprintln!("pattern = \"{}\"", generated_regex);
    eprintln!("callback_function = \"handle_new_generated_regex\"");
    eprintln!("```");

    Ok(())
}
