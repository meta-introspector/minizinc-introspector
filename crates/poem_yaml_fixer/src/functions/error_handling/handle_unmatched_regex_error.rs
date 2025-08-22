use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use serde_json;

// Define the new struct for rich error context
#[derive(Debug, Serialize, Deserialize)]
pub struct RegexMatchErrorContext {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
    pub parsing_state: String,
    pub current_tree_path: Vec<String>,
    pub error_message: String,
}

// Modify the function signature to accept the new context struct
pub fn handle_unmatched_regex_error(context: RegexMatchErrorContext) -> Result<Vec<String>> {
    // Serialize the context to JSON and print it
    let json_context = serde_json::to_string_pretty(&context)?;
    eprintln!("\n--- Regex Match Error Context (JSON) ---");
    eprintln!("{}", json_context);
    eprintln!("----------------------------------------");

    // Original error message for compatibility, can be removed later
    Err(anyhow!("Unmatched regex error for file: \"{}\", message: {}", context.file_path, context.error_message))
}