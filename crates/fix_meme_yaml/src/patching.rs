/// Attempts to patch a problematic YAML line by quoting its content.
/// This is a heuristic function primarily aimed at fixing unquoted strings
/// that cause YAML parsing errors, especially in list items.
///
/// It handles:
/// - Lines starting with '-' (list items): It will quote the content after the hyphen.
/// - Other lines: It will quote the entire line.
///
/// Existing double quotes within the content will be escaped.
pub fn patch_quote_line(line: &str) -> String {
    if line.trim().starts_with("-") {
        // It's a list item. Quote the content after the hyphen.
        let trimmed_line = line.trim_start_matches('-').trim();
        // Escape existing double quotes within the content and then wrap in new quotes.
        format!("- \"{}\"", trimmed_line.replace('"', "\\\"")) // Removed semicolon
    } else {
        // Not a list item. Quote the entire line.
        // Escape existing double quotes within the content and then wrap in new quotes.
        format!("\"{}\"", line.replace('"', "\\\"")) // Removed semicolon
    }
}

// Add more patching functions here as needed.
// For example:
// pub fn patch_remove_backticks(line: &str) -> String { ... }
// pub fn pub fn patch_reformat_list_item(line: &str) -> String { ... }
