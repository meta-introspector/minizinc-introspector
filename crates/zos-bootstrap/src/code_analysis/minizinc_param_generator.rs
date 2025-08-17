use crate::utils::error::Result;
use crate::code_analysis::string_extractor::ExtractedString;
use std::path::PathBuf;
use std::fs;

pub fn generate_minizinc_data_file(extracted_strings: Vec<ExtractedString>, output_path: &PathBuf) -> Result<()> {
    let mut content = String::new();
    content.push_str("extracted_strings = [\n");

    for (i, s) in extracted_strings.iter().enumerate() {
        // Escape backslashes, double quotes, and newlines within the string value for MiniZinc DZN
        let escaped_value = s.string_value
            .replace("\\", "\\\\") // Escape literal backslash with two literal backslashes
            .replace("\"", "\\\"") // Escape literal double quote with escaped double quote
            .replace("\n", "\\n"); // Escape literal newline with escaped newline
        content.push_str(&format!("    \"{}\"", escaped_value)); // Each string literal on its own line
        if i < extracted_strings.len() - 1 {
            content.push_str(",\n"); // Comma and actual newline for array separation
        } else {
            content.push_str("\n"); // Actual newline after the last element
        }
    }
    content.push_str("];\n"); // Closing bracket and actual newline

    fs::write(output_path, content)?;
    println!("Generated MiniZinc data file: {}", output_path.display());
    Ok(())
}

pub fn generate_minizinc_selection_model(output_path: &PathBuf) -> Result<()> {
    let model_content = r###"array[int] of string: extracted_strings;
int: num_strings = length(extracted_strings);

% Decision variable: which string to select
var 1..num_strings: selected_index;

% The selected string
string: selected_string = extracted_strings[selected_index];

solve satisfy;

output ["selected_string = ", selected_string, "\n"];
"###;

    fs::write(output_path, model_content)?;
    println!("Generated MiniZinc selection model: {}", output_path.display());
    Ok(())
}
