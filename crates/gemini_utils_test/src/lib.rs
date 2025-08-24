use gemini_utils::gemini_eprintln;

pub fn log_examples() {
    // Simple message
    gemini_eprintln!("A simple message.");

    // Message with named arguments
    let version = "1.0.0";
    let width = 1024;
    let height = 768;
    gemini_eprintln!("  Version: :version:", version = version);
    gemini_eprintln!("  Width: :width:", width = width);
    gemini_eprintln!("  Height: :height:", height = height);

    // Message with special characters (emojis/keywords)
    let limit = 100;
    gemini_eprintln!("sparklesProcessing events (limited to :limit:)...::newline::", limit = limit);

    // Message with debug formatting (using the workaround)
    let output_file = std::path::PathBuf::from("/tmp/output.txt");
    gemini_eprintln!("Generated Rust code written to: :file_path:", file_path = format!("{:?}", output_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_examples_compiles() {
        // This test primarily checks if the code within log_examples compiles.
        // Output to stderr from gemini_eprintln! is not captured or asserted here.
        log_examples();
    }
}
