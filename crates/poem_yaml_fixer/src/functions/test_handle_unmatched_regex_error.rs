

#[cfg(test)]
use anyhow::Result;
#[cfg(test)]
use crate::functions::error_handling::handle_unmatched_regex_error::handle_unmatched_regex_error;
#[cfg(test)]
use crate::functions::error_handling::handle_unmatched_regex_error::RegexMatchErrorContext; // Added import
#[cfg(test)]
use std::path::PathBuf;

#[test]
fn test_handle_unmatched_regex_error() -> Result<()> {
    let file_path = PathBuf::from("test_file.md");
    let error_message = "No regex matched line: some unmatched line";

    // This function is expected to return an error
    let context = RegexMatchErrorContext {
        file_path: file_path.to_string_lossy().into_owned(),
        line_number: 0, // Dummy value
        line_content: error_message.to_string(),
        context_before: Vec::new(), // New field
        context_after: Vec::new(),  // New field
        parsing_state: "TestState".to_string(), // New field
        current_tree_path: Vec::new(), // New field
        error_message: "Unmatched regex error in test".to_string(), // Updated message
    };
    let result = handle_unmatched_regex_error(context);

    assert!(result.is_err());

    // Optionally, check the error message content
    let err = result.unwrap_err();
    assert!(err.to_string().contains("No regex matched line"));
    assert!(err.to_string().contains("test_file.md"));

    Ok(())
}
