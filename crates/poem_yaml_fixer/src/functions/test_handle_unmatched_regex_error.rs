

//use    crate::functions::error_handling::handle_unmatched_regex_error::handle_unmatched_regex_error;
//use std::path::PathBuf;
//use anyhow::Result;

#[test]
fn test_handle_unmatched_regex_error() -> Result<()> {
    let file_path = PathBuf::from("test_file.md");
    let error_message = "No regex matched line: some unmatched line";

    // This function is expected to return an error
    let result = handle_unmatched_regex_error(&file_path, error_message);

    assert!(result.is_err());

    // Optionally, check the error message content
    let err = result.unwrap_err();
    assert!(err.to_string().contains("No regex matched line"));
    assert!(err.to_string().contains("test_file.md"));

    Ok(())
}
