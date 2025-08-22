use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use regex::Regex;
use std::io::Write;

// Helper function to sanitize text for use as a Rust identifier
fn sanitize_name(text: &str) -> String {
    // Take first 50 characters to keep names manageable
    let mut name: String = text.chars().take(50).collect();
    // Replace non-alphanumeric characters with underscores
    name = name.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
    // Remove leading/trailing underscores
    name = name.trim_matches('_').to_string();
    // Ensure it doesn't start with a number
    if name.starts_with(char::is_numeric) {
        name = format!("_{}", name);
    }
    // Convert to snake_case (approximation)
    // This is a very basic approximation and might not produce perfect snake_case
    // for all inputs, but should be sufficient for generating valid identifiers.
    let mut snake_case_name = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            snake_case_name.push('_');
        }
        snake_case_name.push(c.to_lowercase().next().unwrap_or(c));
    }
    snake_case_name
}

pub fn generate_and_save_new_regex_templates(
    unmatched_lines: &[String],
    current_dir: &PathBuf,
) -> Result<()> {
    let regex_templates_dir = current_dir.join("crates/poem_yaml_fixer/src/regex_templates");
    std::fs::create_dir_all(&regex_templates_dir)?;

    let mut new_module_declarations = Vec::new();

    for line in unmatched_lines {
        let generated_regex_pattern = regex::escape(line);
        let sanitized_name = sanitize_name(line);
        let module_name = format!("regex_template_{}", sanitized_name);
        let function_name = format!("{}_regex", sanitized_name);

        let file_path = regex_templates_dir.join(format!("{}.rs", module_name));
        
        let file_content = format!(
            r#"// {}
use regex::Regex;
use std::collections::HashMap;

pub fn {}() -> (Regex, HashMap<String, String>) {{
    let regex = Regex::new(r\#"{}"\#).unwrap();
    let mut captures = HashMap::new();
    // TODO: Populate captures based on the regex groups if any
    (regex, captures)
}}
"#,
            file_path.display(),
            function_name,
            generated_regex_pattern
        );

        let mut file = std::fs::File::create(&file_path)?;
        file.write_all(file_content.as_bytes())?;
        println!("Created new regex template file: {:?}", file_path);

        new_module_declarations.push(format!("pub mod {};", module_name));
    }

    // Update the regex_templates/mod.rs file
    let mod_rs_path = regex_templates_dir.join("mod.rs");
    let mut mod_rs_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mod_rs_path)?;
    
    for declaration in new_module_declarations {
        writeln!(mod_rs_file, "{}", declaration)?;
    }
    println!("Updated {:?} with new module declarations.", mod_rs_path);

    Ok(())
}
