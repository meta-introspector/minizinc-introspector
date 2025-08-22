use anyhow::Result;
use grex::RegExpBuilder;

pub fn handle_unmatched_regex_error(file_path: &std::path::PathBuf, error_message: &str) -> Result<()> {
    eprintln!("Error fixing {file_path:?}: {error_message}\n");

    let unmatched_line = error_message.trim_start_matches("No regex matched line: ");

    eprintln!("\nAttempting to generate regex for: `{unmatched_line}`");
    let generated_regex = RegExpBuilder::from(&[unmatched_line])
        .build();

    eprintln!("Generated Regex: `{generated_regex}`");
    eprintln!("Please review this regex and add it to your regex config file (e.g., `regex_config.toml`).");
    eprintln!("Example entry:");
    eprintln!("```toml");
    eprintln!("[[regexes]]"); // Added [[regexes]] to indicate array of tables
    eprintln!("name = \"new_generated_regex\""); // Added a placeholder name
    eprintln!("pattern = \"{generated_regex}\"");
    eprintln!("callback_function = \"handle_new_generated_regex\"");
    eprintln!("```");

    Ok(())
}